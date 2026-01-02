mkuse!{use std :: fmt :: Write as _ ;}
mkuse!{use rustc_abi :: { ExternAbi , Integer } ;}
mkuse!{use rustc_data_structures :: base_n :: { ALPHANUMERIC_ONLY , CASE_INSENSITIVE , ToBaseN } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: IntegerExt ;}
mkuse!{use rustc_middle :: ty :: { self , Const , ExistentialPredicate , FloatTy , FnSig , GenericArg , GenericArgKind , GenericArgsRef , IntTy , List , Region , RegionKind , TermKind , Ty , TyCtxt , TypeFoldable , UintTy , } ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: cfi :: typeid :: TypeIdOptions ;}
mkuse!{use crate :: cfi :: typeid :: itanium_cxx_abi :: transform :: { TransformTy , TransformTyOptions } ;}
mkitem!{# [doc = " Options for encode_ty."] pub (crate) type EncodeTyOptions = TypeIdOptions ;}
mkitem!{mkenum!{# [doc = " Substitution dictionary key."] # [derive (Eq , Hash , PartialEq)] pub (crate) enum DictKey < 'tcx > { Ty (Ty < 'tcx > , TyQ) , Region (Region < 'tcx >) , Const (Const < 'tcx >) , Predicate (ExistentialPredicate < 'tcx >) , }}}
mkitem!{mkenum!{# [doc = " Type and extended type qualifiers."] # [derive (Eq , Hash , PartialEq)] pub (crate) enum TyQ { None , Const , Mut , }}}

macro_rules! compress_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compress in module {}", module_path!());
    };
}

mkfn!{
    compress_introspect!();
    # [doc = " Substitutes a component if found in the substitution dictionary (see"] # [doc = " <https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling-compression>)."] fn compress < 'tcx > (dict : & mut FxHashMap < DictKey < 'tcx > , usize > , key : DictKey < 'tcx > , comp : & mut String ,) { match dict . get (& key) { Some (num) => { comp . clear () ; let _ = write ! (comp , "S{}_" , to_seq_id (* num)) ; } None => { dict . insert (key , dict . len ()) ; } } }
}

macro_rules! encode_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_args in module {}", module_path!());
    };
}

mkfn!{
    encode_args_introspect!();
    # [doc = " Encodes args using the Itanium C++ ABI with vendor extended type qualifiers and types for Rust"] # [doc = " types that are not used at the FFI boundary."] fn encode_args < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > , for_def : DefId , has_erased_self : bool , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: new () ; let args : Vec < GenericArg < '_ > > = args . iter () . collect () ; if ! args . is_empty () { s . push ('I') ; let def_generics = tcx . generics_of (for_def) ; for (n , arg) in args . iter () . enumerate () { match arg . kind () { GenericArgKind :: Lifetime (region) => { s . push_str (& encode_region (region , dict)) ; } GenericArgKind :: Type (ty) => { s . push_str (& encode_ty (tcx , ty , dict , options)) ; } GenericArgKind :: Const (c) => { let n = n + (has_erased_self as usize) ; let ct_ty = tcx . type_of (def_generics . param_at (n , tcx) . def_id) . instantiate_identity () ; s . push_str (& encode_const (tcx , c , ct_ty , dict , options)) ; } } } s . push ('E') ; } s }
}

macro_rules! encode_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_const in module {}", module_path!());
    };
}

mkfn!{
    encode_const_introspect!();
    # [doc = " Encodes a const using the Itanium C++ ABI as a literal argument (see"] # [doc = " <https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling.literal>)."] fn encode_const < 'tcx > (tcx : TyCtxt < 'tcx > , ct : Const < 'tcx > , ct_ty : Ty < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: from ('L') ; match ct . kind () { ty :: ConstKind :: Param (..) => { s . push_str (& encode_ty (tcx , ct_ty , dict , options)) ; } ty :: ConstKind :: Value (cv) => { s . push_str (& encode_ty (tcx , cv . ty , dict , options)) ; match cv . ty . kind () { ty :: Int (ity) => { let bits = cv . try_to_bits (tcx , ty :: TypingEnv :: fully_monomorphized ()) . expect ("expected monomorphic const in cfi") ; let val = Integer :: from_int_ty (& tcx , * ity) . size () . sign_extend (bits) as i128 ; if val < 0 { s . push ('n') ; } let _ = write ! (s , "{val}") ; } ty :: Uint (_) => { let val = cv . try_to_bits (tcx , ty :: TypingEnv :: fully_monomorphized ()) . expect ("expected monomorphic const in cfi") ; let _ = write ! (s , "{val}") ; } ty :: Bool => { let val = cv . try_to_bool () . expect ("expected monomorphic const in cfi") ; let _ = write ! (s , "{val}") ; } _ => { bug ! ("encode_const: unexpected type `{:?}`" , cv . ty) ; } } } _ => { bug ! ("encode_const: unexpected kind `{:?}`" , ct . kind ()) ; } } s . push ('E') ; compress (dict , DictKey :: Const (ct) , & mut s) ; s }
}

macro_rules! encode_fnsig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_fnsig in module {}", module_path!());
    };
}

mkfn!{
    encode_fnsig_introspect!();
    # [doc = " Encodes a FnSig using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] fn encode_fnsig < 'tcx > (tcx : TyCtxt < 'tcx > , fn_sig : & FnSig < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : TypeIdOptions ,) -> String { let mut s = String :: from ("F") ; let mut encode_ty_options = EncodeTyOptions :: from_bits (options . bits ()) . unwrap_or_else (| | bug ! ("encode_fnsig: invalid option(s) `{:?}`" , options . bits ())) ; match fn_sig . abi { ExternAbi :: C { .. } => { encode_ty_options . insert (EncodeTyOptions :: GENERALIZE_REPR_C) ; } _ => { encode_ty_options . remove (EncodeTyOptions :: GENERALIZE_REPR_C) ; } } let transform_ty_options = TransformTyOptions :: from_bits (options . bits ()) . unwrap_or_else (| | bug ! ("encode_fnsig: invalid option(s) `{:?}`" , options . bits ())) ; let mut type_folder = TransformTy :: new (tcx , transform_ty_options) ; let ty = fn_sig . output () . fold_with (& mut type_folder) ; s . push_str (& encode_ty (tcx , ty , dict , encode_ty_options)) ; let tys = fn_sig . inputs () ; if ! tys . is_empty () { for ty in tys { let ty = ty . fold_with (& mut type_folder) ; s . push_str (& encode_ty (tcx , ty , dict , encode_ty_options)) ; } if fn_sig . c_variadic { s . push ('z') ; } } else if fn_sig . c_variadic { s . push ('z') ; } else { s . push ('v') } s . push ('E') ; s }
}

macro_rules! encode_predicate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_predicate in module {}", module_path!());
    };
}

mkfn!{
    encode_predicate_introspect!();
    # [doc = " Encodes a predicate using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] fn encode_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , predicate : ty :: PolyExistentialPredicate < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: new () ; match predicate . as_ref () . skip_binder () { ty :: ExistentialPredicate :: Trait (trait_ref) => { let name = encode_ty_name (tcx , trait_ref . def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; s . push_str (& encode_args (tcx , trait_ref . args , trait_ref . def_id , true , dict , options)) ; } ty :: ExistentialPredicate :: Projection (projection) => { let name = encode_ty_name (tcx , projection . def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; s . push_str (& encode_args (tcx , projection . args , projection . def_id , true , dict , options)) ; match projection . term . kind () { TermKind :: Ty (ty) => s . push_str (& encode_ty (tcx , ty , dict , options)) , TermKind :: Const (c) => s . push_str (& encode_const (tcx , c , tcx . type_of (projection . def_id) . instantiate (tcx , projection . args) , dict , options ,)) , } } ty :: ExistentialPredicate :: AutoTrait (def_id) => { let name = encode_ty_name (tcx , * def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; } } ; compress (dict , DictKey :: Predicate (* predicate . as_ref () . skip_binder ()) , & mut s) ; s }
}

macro_rules! encode_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_predicates in module {}", module_path!());
    };
}

mkfn!{
    encode_predicates_introspect!();
    # [doc = " Encodes predicates using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] fn encode_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , predicates : & List < ty :: PolyExistentialPredicate < 'tcx > > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: new () ; let predicates : Vec < ty :: PolyExistentialPredicate < 'tcx > > = predicates . iter () . collect () ; for predicate in predicates { s . push_str (& encode_predicate (tcx , predicate , dict , options)) ; } s }
}

macro_rules! encode_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_region in module {}", module_path!());
    };
}

mkfn!{
    encode_region_introspect!();
    # [doc = " Encodes a region using the Itanium C++ ABI as a vendor extended type."] fn encode_region < 'tcx > (region : Region < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize >) -> String { let mut s = String :: new () ; match region . kind () { RegionKind :: ReBound (debruijn , r) => { s . push_str ("u6regionI") ; let num = debruijn . index () as u64 ; if num > 0 { s . push_str (& to_disambiguator (num)) ; } let _ = write ! (s , "{}" , r . var . index () as u64) ; s . push ('E') ; compress (dict , DictKey :: Region (region) , & mut s) ; } RegionKind :: ReErased => { s . push_str ("u6region") ; compress (dict , DictKey :: Region (region) , & mut s) ; } RegionKind :: ReEarlyParam (..) | RegionKind :: ReLateParam (..) | RegionKind :: ReStatic | RegionKind :: ReError (_) | RegionKind :: ReVar (..) | RegionKind :: RePlaceholder (..) => { bug ! ("encode_region: unexpected `{:?}`" , region . kind ()) ; } } s }
}

macro_rules! encode_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_ty in module {}", module_path!());
    };
}

mkfn!{
    encode_ty_introspect!();
    # [doc = " Encodes a ty:Ty using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] # [instrument (level = "trace" , skip (tcx , dict))] pub (crate) fn encode_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut typeid = String :: new () ; match ty . kind () { ty :: Bool => { typeid . push ('b') ; } ty :: Int (..) | ty :: Uint (..) => { let mut s = String :: from (match ty . kind () { ty :: Int (IntTy :: I8) => "u2i8" , ty :: Int (IntTy :: I16) => "u3i16" , ty :: Int (IntTy :: I32) => "u3i32" , ty :: Int (IntTy :: I64) => "u3i64" , ty :: Int (IntTy :: I128) => "u4i128" , ty :: Int (IntTy :: Isize) => "u5isize" , ty :: Uint (UintTy :: U8) => "u2u8" , ty :: Uint (UintTy :: U16) => "u3u16" , ty :: Uint (UintTy :: U32) => "u3u32" , ty :: Uint (UintTy :: U64) => "u3u64" , ty :: Uint (UintTy :: U128) => "u4u128" , ty :: Uint (UintTy :: Usize) => "u5usize" , _ => bug ! ("encode_ty: unexpected `{:?}`" , ty . kind ()) , }) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Float (float_ty) => { typeid . push_str (match float_ty { FloatTy :: F16 => "Dh" , FloatTy :: F32 => "f" , FloatTy :: F64 => "d" , FloatTy :: F128 => "g" , }) ; } ty :: Char => { let mut s = String :: from ("u4char") ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Str => { let mut s = String :: from ("u3str") ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Never => { let mut s = String :: from ("u5never") ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } _ if ty . is_unit () => { typeid . push ('v') ; } ty :: Tuple (tys) => { let mut s = String :: from ("u5tupleI") ; for ty in tys . iter () { s . push_str (& encode_ty (tcx , ty , dict , options)) ; } s . push ('E') ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Array (ty0 , len) => { let len = len . try_to_target_usize (tcx) . expect ("expected monomorphic const in cfi") ; let mut s = String :: from ("A") ; let _ = write ! (s , "{len}") ; s . push_str (& encode_ty (tcx , * ty0 , dict , options)) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Pat (ty0 , pat) => { let mut s = String :: from ("u3patI") ; s . push_str (& encode_ty (tcx , * ty0 , dict , options)) ; write ! (s , "{:?}" , ** pat) . unwrap () ; s . push ('E') ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Slice (ty0) => { let mut s = String :: from ("u5sliceI") ; s . push_str (& encode_ty (tcx , * ty0 , dict , options)) ; s . push ('E') ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Adt (adt_def , args) => { let mut s = String :: new () ; let def_id = adt_def . did () ; if let Some (cfi_encoding) = tcx . get_attr (def_id , sym :: cfi_encoding) { if let Some (value_str) = cfi_encoding . value_str () { let value_str = value_str . as_str () . trim () ; if ! value_str . is_empty () { s . push_str (value_str) ; let builtin_types = ["v" , "w" , "b" , "c" , "a" , "h" , "s" , "t" , "i" , "j" , "l" , "m" , "x" , "y" , "n" , "o" , "f" , "d" , "e" , "g" , "z" , "Dh" ,] ; if ! builtin_types . contains (& value_str) { compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; } } else { # [allow (rustc :: diagnostic_outside_of_impl , rustc :: untranslatable_diagnostic)] tcx . dcx () . struct_span_err (cfi_encoding . span () , format ! ("invalid `cfi_encoding` for `{:?}`" , ty . kind ()) ,) . emit () ; } } else { bug ! ("encode_ty: invalid `cfi_encoding` for `{:?}`" , ty . kind ()) ; } } else if options . contains (EncodeTyOptions :: GENERALIZE_REPR_C) && adt_def . repr () . c () { let name = tcx . item_name (def_id) . to_string () ; let _ = write ! (s , "{}{}" , name . len () , name) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; } else { let name = encode_ty_name (tcx , def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; s . push_str (& encode_args (tcx , args , def_id , false , dict , options)) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; } typeid . push_str (& s) ; } ty :: Foreign (def_id) => { let mut s = String :: new () ; if let Some (cfi_encoding) = tcx . get_attr (* def_id , sym :: cfi_encoding) { if let Some (value_str) = cfi_encoding . value_str () { if ! value_str . to_string () . trim () . is_empty () { s . push_str (value_str . to_string () . trim ()) ; } else { # [allow (rustc :: diagnostic_outside_of_impl , rustc :: untranslatable_diagnostic)] tcx . dcx () . struct_span_err (cfi_encoding . span () , format ! ("invalid `cfi_encoding` for `{:?}`" , ty . kind ()) ,) . emit () ; } } else { bug ! ("encode_ty: invalid `cfi_encoding` for `{:?}`" , ty . kind ()) ; } } else { let name = tcx . item_name (* def_id) . to_string () ; let _ = write ! (s , "{}{}" , name . len () , name) ; } compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: FnDef (def_id , args) | ty :: Closure (def_id , args) => { let mut s = String :: new () ; let name = encode_ty_name (tcx , * def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; s . push_str (& encode_args (tcx , args , * def_id , false , dict , options)) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: CoroutineClosure (def_id , args) => { let mut s = String :: new () ; let name = encode_ty_name (tcx , * def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; let parent_args = tcx . mk_args (args . as_coroutine_closure () . parent_args ()) ; s . push_str (& encode_args (tcx , parent_args , * def_id , false , dict , options)) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Coroutine (def_id , args , ..) => { let mut s = String :: new () ; let name = encode_ty_name (tcx , * def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; s . push_str (& encode_args (tcx , tcx . mk_args (args . as_coroutine () . parent_args ()) , * def_id , false , dict , options ,)) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Ref (region , ty0 , ..) => { let mut s = String :: new () ; s . push_str ("u3refI") ; s . push_str (& encode_ty (tcx , * ty0 , dict , options)) ; s . push ('E') ; compress (dict , DictKey :: Ty (Ty :: new_imm_ref (tcx , * region , * ty0) , TyQ :: None) , & mut s) ; if ty . is_mutable_ptr () { s = format ! ("{}{}" , "U3mut" , s) ; compress (dict , DictKey :: Ty (ty , TyQ :: Mut) , & mut s) ; } typeid . push_str (& s) ; } ty :: RawPtr (ptr_ty , _mutbl) => { let mut s = String :: new () ; s . push_str (& encode_ty (tcx , * ptr_ty , dict , options)) ; if ! ty . is_mutable_ptr () { s = format ! ("{}{}" , "K" , s) ; compress (dict , DictKey :: Ty (* ptr_ty , TyQ :: Const) , & mut s) ; } ; s = format ! ("{}{}" , "P" , s) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: FnPtr (sig_tys , hdr) => { let mut s = String :: from ("P") ; s . push_str (& encode_fnsig (tcx , & sig_tys . with (* hdr) . skip_binder () , dict , TypeIdOptions :: empty () ,)) ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: UnsafeBinder (_) => { todo ! () } ty :: Dynamic (predicates , region , kind) => { let mut s = String :: from (match kind { ty :: Dyn => "u3dynI" , }) ; s . push_str (& encode_predicates (tcx , predicates , dict , options)) ; s . push_str (& encode_region (* region , dict)) ; s . push ('E') ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Param (..) => { let mut s = String :: from ("u5param") ; compress (dict , DictKey :: Ty (ty , TyQ :: None) , & mut s) ; typeid . push_str (& s) ; } ty :: Alias (..) | ty :: Bound (..) | ty :: Error (..) | ty :: CoroutineWitness (..) | ty :: Infer (..) | ty :: Placeholder (..) => { bug ! ("encode_ty: unexpected `{:?}`" , ty . kind ()) ; } } ; typeid }
}

macro_rules! encode_ty_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function encode_ty_name in module {}", module_path!());
    };
}

mkfn!{
    encode_ty_name_introspect!();
    # [doc = " Encodes a ty:Ty name, including its crate and path disambiguators and names."] fn encode_ty_name (tcx : TyCtxt < '_ > , def_id : DefId) -> String { let mut s = String :: new () ; let mut def_path = tcx . def_path (def_id) ; def_path . data . reverse () ; for disambiguated_data in & def_path . data { s . push ('N') ; s . push_str (match disambiguated_data . data { hir :: definitions :: DefPathData :: Impl => "I" , hir :: definitions :: DefPathData :: ForeignMod => "F" , hir :: definitions :: DefPathData :: TypeNs (..) => "t" , hir :: definitions :: DefPathData :: ValueNs (..) => "v" , hir :: definitions :: DefPathData :: Closure => "C" , hir :: definitions :: DefPathData :: Ctor => "c" , hir :: definitions :: DefPathData :: AnonConst => "k" , hir :: definitions :: DefPathData :: OpaqueTy => "i" , hir :: definitions :: DefPathData :: SyntheticCoroutineBody => "s" , hir :: definitions :: DefPathData :: NestedStatic => "n" , hir :: definitions :: DefPathData :: CrateRoot | hir :: definitions :: DefPathData :: Use | hir :: definitions :: DefPathData :: GlobalAsm | hir :: definitions :: DefPathData :: MacroNs (..) | hir :: definitions :: DefPathData :: OpaqueLifetime (..) | hir :: definitions :: DefPathData :: LifetimeNs (..) | hir :: definitions :: DefPathData :: AnonAssocTy (..) => { bug ! ("encode_ty_name: unexpected `{:?}`" , disambiguated_data . data) ; } }) ; } s . push ('C') ; s . push_str (& to_disambiguator (tcx . stable_crate_id (def_path . krate) . as_u64 ())) ; let crate_name = tcx . crate_name (def_path . krate) . to_string () ; let _ = write ! (s , "{}{}" , crate_name . len () , crate_name) ; def_path . data . reverse () ; for disambiguated_data in & def_path . data { let num = disambiguated_data . disambiguator as u64 ; if num > 0 { s . push_str (& to_disambiguator (num)) ; } let name = disambiguated_data . data . to_string () ; let _ = write ! (s , "{}" , name . len ()) ; if let Some (first) = name . as_bytes () . first () { if first . is_ascii_digit () || * first == b'_' { s . push ('_') ; } } else { bug ! ("encode_ty_name: invalid name `{:?}`" , name) ; } s . push_str (& name) ; } s }
}

macro_rules! to_disambiguator_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_disambiguator in module {}", module_path!());
    };
}

mkfn!{
    to_disambiguator_introspect!();
    # [doc = " Converts a number to a disambiguator (see"] # [doc = " <https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html>)."] fn to_disambiguator (num : u64) -> String { if let Some (num) = num . checked_sub (1) { format ! ("s{}_" , num . to_base (ALPHANUMERIC_ONLY)) } else { "s_" . to_string () } }
}

macro_rules! to_seq_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_seq_id in module {}", module_path!());
    };
}

mkfn!{
    to_seq_id_introspect!();
    # [doc = " Converts a number to a sequence number (see"] # [doc = " <https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangle.seq-id>)."] fn to_seq_id (num : usize) -> String { if let Some (num) = num . checked_sub (1) { (num as u64) . to_base (CASE_INSENSITIVE) . to_uppercase () } else { "" . to_string () } }
}
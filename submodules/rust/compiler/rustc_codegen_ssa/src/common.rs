mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: attrs :: PeImportNameType ;}
mkuse!{use rustc_middle :: ty :: layout :: TyAndLayout ;}
mkuse!{use rustc_middle :: ty :: { self , Instance , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , mir , span_bug } ;}
mkuse!{use rustc_session :: cstore :: { DllCallingConvention , DllImport } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_target :: spec :: Target ;}
mkuse!{use crate :: traits :: * ;}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug)] pub enum IntPredicate { IntEQ , IntNE , IntUGT , IntUGE , IntULT , IntULE , IntSGT , IntSGE , IntSLT , IntSLE , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug)] pub enum RealPredicate { RealPredicateFalse , RealOEQ , RealOGT , RealOGE , RealOLT , RealOLE , RealONE , RealORD , RealUNO , RealUEQ , RealUGT , RealUGE , RealULT , RealULE , RealUNE , RealPredicateTrue , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Debug)] pub enum AtomicRmwBinOp { AtomicXchg , AtomicAdd , AtomicSub , AtomicAnd , AtomicNand , AtomicOr , AtomicXor , AtomicMax , AtomicMin , AtomicUMax , AtomicUMin , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug)] pub enum SynchronizationScope { SingleThread , CrossThread , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Debug)] pub enum TypeKind { Void , Half , Float , Double , X86_FP80 , FP128 , PPC_FP128 , Label , Integer , Function , Struct , Array , Pointer , Vector , Metadata , Token , ScalableVector , BFloat , X86_AMX , }}}
mkmod!{temp_stable_hash_impls, { 
                getname!(temp_stable_hash_impls);
                getsrc!(temp_stable_hash_impls);
                getpath!(temp_stable_hash_impls);
                get_deps!(temp_stable_hash_impls);
                get_crates!(temp_stable_hash_impls);
                mkinclude!(temp_stable_hash_impls);
                mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;}
mkuse!{use crate :: ModuleCodegen ;}
mkitem!{mkimpl!{impl < HCX , M > HashStable < HCX > for ModuleCodegen < M > { fn hash_stable (& self , _ : & mut HCX , _ : & mut StableHasher) { } }}} 
            }}

macro_rules! build_langcall_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_langcall in module {}", module_path!());
    };
}

mkfn!{
    build_langcall_introspect!();
    pub (crate) fn build_langcall < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & Bx , span : Span , li : LangItem ,) -> (Bx :: FnAbiOfResult , Bx :: Value , Instance < 'tcx >) { let tcx = bx . tcx () ; let def_id = tcx . require_lang_item (li , span) ; let instance = ty :: Instance :: mono (tcx , def_id) ; (bx . fn_abi_of_instance (instance , ty :: List :: empty ()) , bx . get_fn_addr (instance) , instance) }
}

macro_rules! shift_mask_val_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shift_mask_val in module {}", module_path!());
    };
}

mkfn!{
    shift_mask_val_introspect!();
    pub (crate) fn shift_mask_val < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , llty : Bx :: Type , mask_llty : Bx :: Type , invert : bool ,) -> Bx :: Value { let kind = bx . type_kind (llty) ; match kind { TypeKind :: Integer => { let val = bx . int_width (llty) - 1 ; if invert { bx . const_int (mask_llty , ! val as i64) } else { bx . const_uint (mask_llty , val) } } TypeKind :: Vector => { let mask = shift_mask_val (bx , bx . element_type (llty) , bx . element_type (mask_llty) , invert) ; bx . vector_splat (bx . vector_length (mask_llty) , mask) } _ => bug ! ("shift_mask_val: expected Integer or Vector, found {:?}" , kind) , } }
}

macro_rules! asm_const_to_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function asm_const_to_str in module {}", module_path!());
    };
}

mkfn!{
    asm_const_to_str_introspect!();
    pub fn asm_const_to_str < 'tcx > (tcx : TyCtxt < 'tcx > , sp : Span , const_value : mir :: ConstValue , ty_and_layout : TyAndLayout < 'tcx > ,) -> String { let mir :: ConstValue :: Scalar (scalar) = const_value else { span_bug ! (sp , "expected Scalar for promoted asm const, but got {:#?}" , const_value) } ; let value = scalar . assert_scalar_int () . to_bits (ty_and_layout . size) ; match ty_and_layout . ty . kind () { ty :: Uint (_) => value . to_string () , ty :: Int (int_ty) => match int_ty . normalize (tcx . sess . target . pointer_width) { ty :: IntTy :: I8 => (value as i8) . to_string () , ty :: IntTy :: I16 => (value as i16) . to_string () , ty :: IntTy :: I32 => (value as i32) . to_string () , ty :: IntTy :: I64 => (value as i64) . to_string () , ty :: IntTy :: I128 => (value as i128) . to_string () , ty :: IntTy :: Isize => unreachable ! () , } , _ => span_bug ! (sp , "asm const has bad type {}" , ty_and_layout . ty) , } }
}

macro_rules! is_mingw_gnu_toolchain_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_mingw_gnu_toolchain in module {}", module_path!());
    };
}

mkfn!{
    is_mingw_gnu_toolchain_introspect!();
    pub fn is_mingw_gnu_toolchain (target : & Target) -> bool { target . vendor == "pc" && target . os == "windows" && target . env == "gnu" && target . abi . is_empty () }
}

macro_rules! i686_decorated_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i686_decorated_name in module {}", module_path!());
    };
}

mkfn!{
    i686_decorated_name_introspect!();
    pub fn i686_decorated_name (dll_import : & DllImport , mingw : bool , disable_name_mangling : bool , force_fully_decorated : bool ,) -> String { let name = dll_import . name . as_str () ; let (add_prefix , add_suffix) = match (force_fully_decorated , dll_import . import_name_type) { (_ , Some (PeImportNameType :: NoPrefix)) => (false , true) , (false , Some (PeImportNameType :: Undecorated)) => (false , false) , _ => (true , true) , } ; let mut decorated_name = String :: with_capacity (name . len () + 6) ; if disable_name_mangling { decorated_name . push ('\x01') ; } let prefix = if add_prefix && dll_import . is_fn { match dll_import . calling_convention { DllCallingConvention :: C | DllCallingConvention :: Vectorcall (_) => None , DllCallingConvention :: Stdcall (_) => (! mingw || dll_import . import_name_type == Some (PeImportNameType :: Decorated)) . then_some ('_') , DllCallingConvention :: Fastcall (_) => Some ('@') , } } else if ! dll_import . is_fn && ! mingw { Some ('_') } else { None } ; if let Some (prefix) = prefix { decorated_name . push (prefix) ; } decorated_name . push_str (name) ; if add_suffix && dll_import . is_fn { use std :: fmt :: Write ; match dll_import . calling_convention { DllCallingConvention :: C => { } DllCallingConvention :: Stdcall (arg_list_size) | DllCallingConvention :: Fastcall (arg_list_size) => { write ! (& mut decorated_name , "@{arg_list_size}") . unwrap () ; } DllCallingConvention :: Vectorcall (arg_list_size) => { write ! (& mut decorated_name , "@@{arg_list_size}") . unwrap () ; } } } decorated_name }
}
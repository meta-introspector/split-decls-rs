mkuse!{use hir :: HirId ;}
mkuse!{use rustc_abi :: Primitive :: Pointer ;}
mkuse!{use rustc_abi :: VariantIdx ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: struct_span_code_err ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_index :: Idx ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutError , SizeSkeleton } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use tracing :: trace ;}

macro_rules! unpack_option_like_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unpack_option_like in module {}", module_path!());
    };
}

mkfn!{
    unpack_option_like_introspect!();
    # [doc = " If the type is `Option<T>`, it will return `T`, otherwise"] # [doc = " the type itself. Works on most `Option`-like types."] fn unpack_option_like < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Ty < 'tcx > { let ty :: Adt (def , args) = * ty . kind () else { return ty } ; if def . variants () . len () == 2 && ! def . repr () . c () && def . repr () . int . is_none () { let data_idx ; let one = VariantIdx :: new (1) ; let zero = VariantIdx :: ZERO ; if def . variant (zero) . fields . is_empty () { data_idx = one ; } else if def . variant (one) . fields . is_empty () { data_idx = zero ; } else { return ty ; } if def . variant (data_idx) . fields . len () == 1 { return def . variant (data_idx) . single_field () . ty (tcx , args) ; } } ty }
}

macro_rules! skeleton_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function skeleton_string in module {}", module_path!());
    };
}

mkfn!{
    skeleton_string_introspect!();
    # [doc = " Try to display a sensible error with as much information as possible."] fn skeleton_string < 'tcx > (ty : Ty < 'tcx > , sk : Result < SizeSkeleton < 'tcx > , & 'tcx LayoutError < 'tcx > > ,) -> String { match sk { Ok (SizeSkeleton :: Pointer { tail , .. }) => format ! ("pointer to `{tail}`") , Ok (SizeSkeleton :: Known (size , _)) => { if let Some (v) = u128 :: from (size . bytes ()) . checked_mul (8) { format ! ("{v} bits") } else { bug ! ("{:?} overflow for u128" , size) } } Ok (SizeSkeleton :: Generic (size)) => { format ! ("generic size {size}") } Err (LayoutError :: TooGeneric (bad)) => { if * bad == ty { "this type does not have a fixed size" . to_owned () } else { format ! ("size can vary because of {bad}") } } Err (err) => err . to_string () , } }
}

macro_rules! check_transmute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_transmute in module {}", module_path!());
    };
}

mkfn!{
    check_transmute_introspect!();
    fn check_transmute < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , from : Ty < 'tcx > , to : Ty < 'tcx > , hir_id : HirId ,) { let span = | | tcx . hir_span (hir_id) ; let normalize = | ty | { if let Ok (ty) = tcx . try_normalize_erasing_regions (typing_env , ty) { ty } else { Ty :: new_error_with_message (tcx , span () , format ! ("tried to normalize non-wf type {ty:#?} in check_transmute") ,) } } ; let from = normalize (from) ; let to = normalize (to) ; trace ! (? from , ? to) ; if from == to { return ; } let sk_from = SizeSkeleton :: compute (from , tcx , typing_env) ; let sk_to = SizeSkeleton :: compute (to , tcx , typing_env) ; trace ! (? sk_from , ? sk_to) ; if let Ok (sk_from) = sk_from && let Ok (sk_to) = sk_to { if sk_from . same_size (sk_to) { return ; } let from = unpack_option_like (tcx , from) ; if let ty :: FnDef (..) = from . kind () && let SizeSkeleton :: Known (size_to , _) = sk_to && size_to == Pointer (tcx . data_layout . instruction_address_space) . size (& tcx) { struct_span_code_err ! (tcx . sess . dcx () , span () , E0591 , "can't transmute zero-sized type") . with_note (format ! ("source type: {from}")) . with_note (format ! ("target type: {to}")) . with_help ("cast with `as` to a pointer instead") . emit () ; return ; } } let mut err = struct_span_code_err ! (tcx . sess . dcx () , span () , E0512 , "cannot transmute between types of different sizes, or dependently-sized types") ; if from == to { err . note (format ! ("`{from}` does not have a fixed size")) ; err . emit () ; } else { err . note (format ! ("source type: `{}` ({})" , from , skeleton_string (from , sk_from))) ; err . note (format ! ("target type: `{}` ({})" , to , skeleton_string (to , sk_to))) ; err . emit () ; } }
}

macro_rules! check_transmutes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_transmutes in module {}", module_path!());
    };
}

mkfn!{
    check_transmutes_introspect!();
    pub (crate) fn check_transmutes (tcx : TyCtxt < '_ > , owner : LocalDefId) { assert ! (! tcx . is_typeck_child (owner . to_def_id ())) ; let typeck_results = tcx . typeck (owner) ; let None = typeck_results . tainted_by_errors else { return } ; let typing_env = ty :: TypingEnv :: post_analysis (tcx , owner) ; for & (from , to , hir_id) in & typeck_results . transmutes_to_check { check_transmute (tcx , typing_env , from , to , hir_id) ; } }
}
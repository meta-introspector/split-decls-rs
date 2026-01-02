mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , mir } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: interpret :: InterpCx ;}
mkmod!{dummy_machine, { 
                getname!(dummy_machine);
                getsrc!(dummy_machine);
                getpath!(dummy_machine);
                get_deps!(dummy_machine);
                get_crates!(dummy_machine);
                mkinclude!(dummy_machine);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{eval_queries, { 
                getname!(eval_queries);
                getsrc!(eval_queries);
                getpath!(eval_queries);
                get_deps!(eval_queries);
                get_crates!(eval_queries);
                mkinclude!(eval_queries);
                 
            }}
mkmod!{fn_queries, { 
                getname!(fn_queries);
                getsrc!(fn_queries);
                getpath!(fn_queries);
                get_deps!(fn_queries);
                get_crates!(fn_queries);
                mkinclude!(fn_queries);
                 
            }}
mkmod!{machine, { 
                getname!(machine);
                getsrc!(machine);
                getpath!(machine);
                get_deps!(machine);
                get_crates!(machine);
                mkinclude!(machine);
                 
            }}
mkmod!{valtrees, { 
                getname!(valtrees);
                getsrc!(valtrees);
                getpath!(valtrees);
                get_deps!(valtrees);
                get_crates!(valtrees);
                mkinclude!(valtrees);
                 
            }}
mkuse!{pub use self :: dummy_machine :: * ;}
mkuse!{pub use self :: error :: * ;}
mkuse!{pub use self :: eval_queries :: * ;}
mkuse!{pub use self :: fn_queries :: * ;}
mkuse!{pub use self :: machine :: * ;}
mkuse!{pub (crate) use self :: valtrees :: { eval_to_valtree , valtree_to_const_value } ;}
mkitem!{const VALTREE_MAX_NODES : usize = 100000 ;}

macro_rules! try_destructure_mir_constant_for_user_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_destructure_mir_constant_for_user_output in module {}", module_path!());
    };
}

mkfn!{
    try_destructure_mir_constant_for_user_output_introspect!();
    # [instrument (skip (tcx) , level = "debug")] pub (crate) fn try_destructure_mir_constant_for_user_output < 'tcx > (tcx : TyCtxt < 'tcx > , val : mir :: ConstValue , ty : Ty < 'tcx > ,) -> Option < mir :: DestructuredConstant < 'tcx > > { let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let (ecx , op) = mk_eval_cx_for_const_val (tcx . at (rustc_span :: DUMMY_SP) , typing_env , val , ty) ? ; let (field_count , variant , down) = match ty . kind () { ty :: Array (_ , len) => (len . try_to_target_usize (tcx) ? as usize , None , op) , ty :: Adt (def , _) if def . variants () . is_empty () => { return None ; } ty :: Adt (def , _) => { let variant = ecx . read_discriminant (& op) . discard_err () ? ; let down = ecx . project_downcast (& op , variant) . discard_err () ? ; (def . variants () [variant] . fields . len () , Some (variant) , down) } ty :: Tuple (args) => (args . len () , None , op) , _ => bug ! ("cannot destructure mir constant {:?}" , val) , } ; let fields_iter = (0 .. field_count) . map (| i | { let field_op = ecx . project_field (& down , FieldIdx :: from_usize (i)) . discard_err () ? ; let val = op_to_const (& ecx , & field_op , true) ; Some ((val , field_op . layout . ty)) }) . collect :: < Option < Vec < _ > > > () ? ; let fields = tcx . arena . alloc_from_iter (fields_iter) ; Some (mir :: DestructuredConstant { variant , fields }) }
}

macro_rules! tag_for_variant_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tag_for_variant_provider in module {}", module_path!());
    };
}

mkfn!{
    tag_for_variant_provider_introspect!();
    # [doc = " Computes the tag (if any) for a given type and variant."] # [instrument (skip (tcx) , level = "debug")] pub fn tag_for_variant_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , (Ty < 'tcx > , VariantIdx) > ,) -> Option < ty :: ScalarInt > { let (ty , variant_index) = key . value ; assert ! (ty . is_enum ()) ; let ecx = InterpCx :: new (tcx , DUMMY_SP , key . typing_env , crate :: const_eval :: DummyMachine) ; let layout = ecx . layout_of (ty) . unwrap () ; ecx . tag_for_variant (layout , variant_index) . unwrap () . map (| (tag , _tag_field) | tag) }
}
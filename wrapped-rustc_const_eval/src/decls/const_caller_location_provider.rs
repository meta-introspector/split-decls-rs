macro_rules! deps {
    () => {
        CanAccessMutGlobal!();
        InternKind!();
    };
}

macro_rules! const_caller_location_provider {
    () => {
        deps!();
        pub (crate) fn const_caller_location_provider (tcx : TyCtxt < '_ > , file : Symbol , line : u32 , col : u32 ,) -> mir :: ConstValue { trace ! ("const_caller_location: {}:{}:{}" , file , line , col) ; let mut ecx = mk_eval_cx_to_read_const_val (tcx , rustc_span :: DUMMY_SP , ty :: TypingEnv :: fully_monomorphized () , CanAccessMutGlobal :: No ,) ; let loc_place = alloc_caller_location (& mut ecx , file , line , col) ; if intern_const_alloc_recursive (& mut ecx , InternKind :: Constant , & loc_place) . is_err () { bug ! ("intern_const_alloc_recursive should not error in this case") } mir :: ConstValue :: Scalar (Scalar :: from_maybe_pointer (loc_place . ptr () , & tcx)) }
    };
}

const_caller_location_provider!()
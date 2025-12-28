macro_rules! deps {
    () => {
        CompileTimeMachine!();
        MemoryKind!();
        InterpCx!();
        CheckAlignment!();
        CanAccessMutGlobal!();
    };
}

macro_rules! validate_scalar_in_layout {
    () => {
        deps!();
        pub (crate) fn validate_scalar_in_layout < 'tcx > (tcx : TyCtxt < 'tcx > , scalar : ScalarInt , ty : Ty < 'tcx > ,) -> bool { let machine = CompileTimeMachine :: new (CanAccessMutGlobal :: No , CheckAlignment :: Error) ; let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let mut cx = InterpCx :: new (tcx , DUMMY_SP , typing_env , machine) ; let Ok (layout) = cx . layout_of (ty) else { bug ! ("could not compute layout of {scalar:?}:{ty:?}") } ; let allocated = cx . allocate (layout , MemoryKind :: Stack) . expect ("OOM: failed to allocate for uninit check") ; cx . write_scalar (scalar , & allocated) . unwrap () ; cx . validate_operand (& allocated . into () , false , false ,) . discard_err () . is_some () }
    };
}

validate_scalar_in_layout!();
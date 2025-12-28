macro_rules! deps {
    () => {
        InterpCx!();
        CanAccessMutGlobal!();
        CheckAlignment!();
        CompileTimeMachine!();
        InterpretationResult!();
    };
}

macro_rules! eval_in_interpreter {
    () => {
        deps!();
        fn eval_in_interpreter < 'tcx , R : InterpretationResult < 'tcx > > (tcx : TyCtxt < 'tcx > , cid : GlobalId < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> Result < R , ErrorHandled > { let def = cid . instance . def . def_id () ; let is_static = tcx . is_static (def) ; let mut ecx = InterpCx :: new (tcx , tcx . def_span (def) , typing_env , CompileTimeMachine :: new (CanAccessMutGlobal :: from (is_static) , CheckAlignment :: Error) ,) ; let res = ecx . load_mir (cid . instance . def , cid . promoted) ; res . and_then (| body | eval_body_using_ecx (& mut ecx , cid , body)) . report_err () . map_err (| error | report_eval_error (& ecx , cid , error)) }
    };
}

eval_in_interpreter!();
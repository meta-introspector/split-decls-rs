macro_rules! deps {
    () => {
        Error!();
        EvaluationWaiting!();
        Reader!();
    };
}

macro_rules! EvaluationState {
    () => {
        deps!();
        # [derive (Debug)] enum EvaluationState < R : Reader > { Start (Option < u64 >) , Ready , Error (Error) , Complete , Waiting (EvaluationWaiting < R >) , }
    };
}

EvaluationState!()
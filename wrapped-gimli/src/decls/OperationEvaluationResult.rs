macro_rules! deps {
    () => {
        EvaluationWaiting!();
        Piece!();
        Location!();
        EvaluationResult!();
        Reader!();
    };
}

macro_rules! OperationEvaluationResult {
    () => {
        deps!();
        # [derive (Debug)] enum OperationEvaluationResult < R : Reader > { Piece , Incomplete , Complete { location : Location < R > } , Waiting (EvaluationWaiting < R > , EvaluationResult < R >) , }
    };
}

OperationEvaluationResult!()
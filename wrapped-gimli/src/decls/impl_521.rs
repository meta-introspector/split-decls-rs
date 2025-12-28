macro_rules! deps {
    () => {
        Reader!();
        Piece!();
        Value!();
        EvaluationStorage!();
        Result!();
        StoreOnHeap!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        # [cfg (feature = "read")] impl < R : Reader > EvaluationStorage < R > for StoreOnHeap { type Stack = Vec < Value > ; type ExpressionStack = Vec < (R , R) > ; type Result = Vec < Piece < R > > ; }
    };
}

impl_521!();
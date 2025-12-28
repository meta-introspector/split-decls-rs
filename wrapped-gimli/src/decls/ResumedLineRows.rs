macro_rules! deps {
    () => {
        Reader!();
        LineRows!();
        CompleteLineProgram!();
    };
}

macro_rules! ResumedLineRows {
    () => {
        deps!();
        type ResumedLineRows < 'program , R , Offset = < R as Reader > :: Offset > = LineRows < R , & 'program CompleteLineProgram < R , Offset > , Offset > ;
    };
}

ResumedLineRows!()
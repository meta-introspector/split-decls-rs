macro_rules! deps {
    () => {
        IncompleteLineProgram!();
        Reader!();
        LineRows!();
    };
}

macro_rules! OneShotLineRows {
    () => {
        deps!();
        type OneShotLineRows < R , Offset = < R as Reader > :: Offset > = LineRows < R , IncompleteLineProgram < R , Offset > , Offset > ;
    };
}

OneShotLineRows!()
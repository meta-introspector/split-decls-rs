macro_rules! deps {
    () => {
        Reader!();
        LineRows!();
        IncompleteLineProgram!();
    };
}

macro_rules! OneShotLineRows {
    () => {
        deps!();
        type OneShotLineRows < R , Offset = < R as Reader > :: Offset > = LineRows < R , IncompleteLineProgram < R , Offset > , Offset > ;
    };
}

OneShotLineRows!();
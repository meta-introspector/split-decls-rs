macro_rules! deps {
    () => {
        Token!();
        Hunk!();
    };
}

macro_rules! Postprocessor {
    () => {
        deps!();
        struct Postprocessor < 'a , H > { added : & 'a mut [bool] , removed : & 'a [bool] , tokens : & 'a [Token] , hunk : Hunk , heuristic : & 'a mut H , }
    };
}

Postprocessor!();
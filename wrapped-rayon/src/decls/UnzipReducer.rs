macro_rules! deps {
    () => {
        Reducer!();
    };
}

macro_rules! UnzipReducer {
    () => {
        deps!();
        # [doc = " `Reducer` that unzips into two other `Reducer`s"] struct UnzipReducer < RA , RB > { left : RA , right : RB , }
    };
}

UnzipReducer!();
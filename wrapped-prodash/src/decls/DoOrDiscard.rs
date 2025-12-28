macro_rules! deps {
    () => {
        Either!();
        Discard!();
        Progress!();
    };
}

macro_rules! DoOrDiscard {
    () => {
        deps!();
        # [doc = " An implementation of `Progress` which can be created easily from `Option<impl Progress>`."] pub struct DoOrDiscard < T > (Either < T , Discard >) ;
    };
}

DoOrDiscard!()
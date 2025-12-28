macro_rules! deps {
    () => {
        Analysis!();
    };
}

macro_rules! analysis_is_send {
    () => {
        deps!();
        # [test] fn analysis_is_send () { fn is_send < T : Send > () { } is_send :: < Analysis > () ; }
    };
}

analysis_is_send!()
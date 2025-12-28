macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! extend_panic {
    () => {
        deps!();
        # [inline (never)] # [cold] # [track_caller] fn extend_panic () { panic ! ("ArrayVec: capacity exceeded in extend/from_iter") ; }
    };
}

extend_panic!();
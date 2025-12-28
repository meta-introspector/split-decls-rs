macro_rules! deps {
    () => {
        AndNot!();
    };
}

macro_rules! macro_15 {
    () => {
        deps!();
        fwd_binop_x2 ! (AndNot , andnot) ;
    };
}

macro_15!()
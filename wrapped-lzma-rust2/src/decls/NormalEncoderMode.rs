macro_rules! deps {
    () => {
        Optimum!();
    };
}

macro_rules! NormalEncoderMode {
    () => {
        deps!();
        pub (crate) struct NormalEncoderMode { opts : Vec < Optimum > , opt_cur : usize , opt_end : usize , }
    };
}

NormalEncoderMode!()
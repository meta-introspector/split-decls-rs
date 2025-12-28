macro_rules! deps {
    () => {
        Md5Core!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Default for Md5Core { # [inline] fn default () -> Self { Self { block_len : 0 , state : consts :: STATE_INIT , } } }
    };
}

impl_8!()
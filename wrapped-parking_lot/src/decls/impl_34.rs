macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Default for Once { # [inline] fn default () -> Once { Once :: new () } }
    };
}

impl_34!()
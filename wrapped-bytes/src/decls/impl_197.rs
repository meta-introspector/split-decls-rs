macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl Default for BytesMut { # [inline] fn default () -> BytesMut { BytesMut :: new () } }
    };
}

impl_197!();
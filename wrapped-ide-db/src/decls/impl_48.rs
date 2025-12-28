macro_rules! deps {
    () => {
        MiniCore!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > MiniCore < 'a > { # [inline] pub fn new (minicore : & 'a str) -> Self { Self (minicore) } # [inline] pub const fn default () -> Self { Self (test_utils :: MiniCore :: RAW_SOURCE) } }
    };
}

impl_48!()
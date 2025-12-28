macro_rules! deps {
    () => {
        AttributesSection!();
    };
}

macro_rules! impl_1169 {
    () => {
        deps!();
        impl < 'data > AttributesSection < 'data > { # [doc = " Create a new attributes section."] pub fn new () -> Self { Self :: default () } }
    };
}

impl_1169!()
macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Builder { Builder { noncontiguous : noncontiguous :: Builder :: new () , dense_depth : 2 , byte_classes : true , } } }
    };
}

impl_75!();
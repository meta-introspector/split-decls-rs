macro_rules! deps {
    () => {
        Builder!();
        StartKind!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Builder { Builder { noncontiguous : noncontiguous :: Builder :: new () , start_kind : StartKind :: Unanchored , byte_classes : true , } } }
    };
}

impl_61!();
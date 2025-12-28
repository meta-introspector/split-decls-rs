macro_rules! deps {
    () => {
        ParserBuilder!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Default for ParserBuilder { fn default () -> ParserBuilder { ParserBuilder :: new () } }
    };
}

impl_8!();
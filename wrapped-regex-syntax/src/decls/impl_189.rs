macro_rules! deps {
    () => {
        TranslatorBuilder!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl Default for TranslatorBuilder { fn default () -> TranslatorBuilder { TranslatorBuilder :: new () } }
    };
}

impl_189!()
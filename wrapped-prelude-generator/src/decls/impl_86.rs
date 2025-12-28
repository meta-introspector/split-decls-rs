macro_rules! deps {
    () => {
        GemConfig!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Default for GemConfig { fn default () -> Self { GemConfig { gem : Vec :: new () } } }
    };
}

impl_86!()
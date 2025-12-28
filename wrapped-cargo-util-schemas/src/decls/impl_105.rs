macro_rules! deps {
    () => {
        TomlInheritedField!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl Default for TomlInheritedField { fn default () -> Self { Self :: new () } }
    };
}

impl_105!();
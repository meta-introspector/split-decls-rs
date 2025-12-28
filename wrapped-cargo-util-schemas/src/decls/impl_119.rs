macro_rules! deps {
    () => {
        TomlDetailedDependency!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < P : Clone > Default for TomlDetailedDependency < P > { fn default () -> Self { Self { version : Default :: default () , registry : Default :: default () , registry_index : Default :: default () , path : Default :: default () , base : Default :: default () , git : Default :: default () , branch : Default :: default () , tag : Default :: default () , rev : Default :: default () , features : Default :: default () , optional : Default :: default () , default_features : Default :: default () , default_features2 : Default :: default () , package : Default :: default () , public : Default :: default () , artifact : Default :: default () , lib : Default :: default () , target : Default :: default () , _unused_keys : Default :: default () , } } }
    };
}

impl_119!()
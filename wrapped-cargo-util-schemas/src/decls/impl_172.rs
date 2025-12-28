macro_rules! deps {
    () => {
        TomlLint!();
        TomlLintLevel!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl TomlLint { pub fn level (& self) -> TomlLintLevel { match self { Self :: Level (level) => * level , Self :: Config (config) => config . level , } } pub fn priority (& self) -> i8 { match self { Self :: Level (_) => 0 , Self :: Config (config) => config . priority , } } pub fn config (& self) -> Option < & toml :: Table > { match self { Self :: Level (_) => None , Self :: Config (config) => Some (& config . config) , } } }
    };
}

impl_172!();
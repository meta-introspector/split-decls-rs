macro_rules! deps {
    () => {
        NonSubstitution!();
    };
}

macro_rules! macro_103 {
    () => {
        deps!();
        define_handle ! { # [doc = " A reference to a parsed `<prefix>` production."] pub enum PrefixHandle { # [doc = " A handle to some `<prefix>` component that isn't by itself"] # [doc = " substitutable; instead, it's only substitutable *with* its parent"] # [doc = " component."] extra NonSubstitution (NonSubstitution) , } }
    };
}

macro_103!();
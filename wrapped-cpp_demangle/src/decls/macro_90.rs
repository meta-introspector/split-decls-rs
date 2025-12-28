macro_rules! deps {
    () => {
        UnscopedTemplateName!();
        NonSubstitution!();
    };
}

macro_rules! macro_90 {
    () => {
        deps!();
        define_handle ! { # [doc = " A handle to an `UnscopedTemplateName`."] pub enum UnscopedTemplateNameHandle { # [doc = " A handle to some `<unscoped-name>` component that isn't by itself"] # [doc = " substitutable."] extra NonSubstitution (NonSubstitution) , } }
    };
}

macro_90!();
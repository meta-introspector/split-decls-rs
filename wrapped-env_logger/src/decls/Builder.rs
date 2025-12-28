macro_rules! deps {
    () => {
        ConfigurableFormat!();
        FormatFn!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct Builder { pub (crate) default_format : ConfigurableFormat , pub (crate) custom_format : Option < FormatFn > , built : bool , }
    };
}

Builder!()
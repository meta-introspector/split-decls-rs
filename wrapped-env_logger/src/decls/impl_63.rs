macro_rules! deps {
    () => {
        Formatter!();
        WriteStyle!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        # [cfg (feature = "color")] impl Formatter { # [doc = " Get the default [`style::Style`] for the given level."] # [doc = ""] # [doc = " The style can be used to print other values besides the level."] # [doc = ""] # [doc = " See [`style`] for how to adapt it to the styling crate of your choice"] pub fn default_level_style (& self , level : Level) -> style :: Style { if self . write_style == WriteStyle :: Never { style :: Style :: new () } else { match level { Level :: Trace => style :: AnsiColor :: Cyan . on_default () , Level :: Debug => style :: AnsiColor :: Blue . on_default () , Level :: Info => style :: AnsiColor :: Green . on_default () , Level :: Warn => style :: AnsiColor :: Yellow . on_default () , Level :: Error => style :: AnsiColor :: Red . on_default () . effects (style :: Effects :: BOLD) , } } } }
    };
}

impl_63!()
macro_rules! deps {
    () => {
        Error!();
        Usage!();
        Styles!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        # [doc = " Reflection"] impl Styles { # [doc = " General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]"] # [inline (always)] pub const fn get_header (& self) -> & Style { & self . header } # [doc = " Error heading"] # [inline (always)] pub const fn get_error (& self) -> & Style { & self . error } # [doc = " Usage heading"] # [inline (always)] pub const fn get_usage (& self) -> & Style { & self . usage } # [doc = " Literal command-line syntax, e.g. `--help`"] # [inline (always)] pub const fn get_literal (& self) -> & Style { & self . literal } # [doc = " Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]"] # [inline (always)] pub const fn get_placeholder (& self) -> & Style { & self . placeholder } # [doc = " Highlight suggested usage"] # [inline (always)] pub const fn get_valid (& self) -> & Style { & self . valid } # [doc = " Highlight invalid usage"] # [inline (always)] pub const fn get_invalid (& self) -> & Style { & self . invalid } # [doc = " Highlight all specified contexts, e.g. `[default: false]`"] # [doc = ""] # [doc = " To specialize the style of the value within the context, see [`Styles::context_value`]"] # [inline (always)] pub const fn get_context (& self) -> & Style { & self . context } # [doc = " Highlight values within all of the context, e.g. the `false` in `[default: false]`"] # [doc = ""] # [doc = " If not explicitly set, falls back to `context`'s style."] # [inline (always)] pub const fn get_context_value (& self) -> & Style { match & self . context_value { Some (s) => s , None => & self . context , } } }
    };
}

impl_373!();
macro_rules! deps {
    () => {
        Styles!();
        Error!();
        Usage!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl Styles { # [doc = " No terminal styling"] pub const fn plain () -> Self { Self { header : Style :: new () , error : Style :: new () , usage : Style :: new () , literal : Style :: new () , placeholder : Style :: new () , valid : Style :: new () , invalid : Style :: new () , context : Style :: new () , context_value : None , } } # [doc = " Default terminal styling"] pub const fn styled () -> Self { # [cfg (feature = "color")] { Self { header : Style :: new () . bold () . underline () , error : Style :: new () . fg_color (Some (Color :: Ansi (AnsiColor :: Red))) . bold () , usage : Style :: new () . bold () . underline () , literal : Style :: new () . bold () , placeholder : Style :: new () , valid : Style :: new () . fg_color (Some (Color :: Ansi (AnsiColor :: Green))) , invalid : Style :: new () . fg_color (Some (Color :: Ansi (AnsiColor :: Yellow))) , context : Style :: new () , context_value : None , } } # [cfg (not (feature = "color"))] { Self :: plain () } } # [doc = " General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]"] # [inline] pub const fn header (mut self , style : Style) -> Self { self . header = style ; self } # [doc = " Error heading"] # [inline] pub const fn error (mut self , style : Style) -> Self { self . error = style ; self } # [doc = " Usage heading"] # [inline] pub const fn usage (mut self , style : Style) -> Self { self . usage = style ; self } # [doc = " Literal command-line syntax, e.g. `--help`"] # [inline] pub const fn literal (mut self , style : Style) -> Self { self . literal = style ; self } # [doc = " Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]"] # [inline] pub const fn placeholder (mut self , style : Style) -> Self { self . placeholder = style ; self } # [doc = " Highlight suggested usage"] # [inline] pub const fn valid (mut self , style : Style) -> Self { self . valid = style ; self } # [doc = " Highlight invalid usage"] # [inline] pub const fn invalid (mut self , style : Style) -> Self { self . invalid = style ; self } # [doc = " Highlight all specified contexts, e.g. `[default: false]`"] # [doc = ""] # [doc = " To specialize the style of the value within the context, see [`Styles::context_value`]"] # [inline] pub const fn context (mut self , style : Style) -> Self { self . context = style ; self } # [doc = " Highlight values within all of the context, e.g. the `false` in `[default: false]`"] # [doc = ""] # [doc = " If not explicitly set, falls back to `context`'s style."] # [inline] pub const fn context_value (mut self , style : Style) -> Self { self . context_value = Some (style) ; self } }
    };
}

impl_372!();
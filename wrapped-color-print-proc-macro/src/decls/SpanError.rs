macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! SpanError {
    () => {
        deps!();
        # [doc = " An error with an optional span. Most errors will have a span, the only exception is on a"] # [doc = " [`Error::Parse`], which can occur in the [`get_args_and_format_string()`] function."] # [doc = ""] # [doc = " [`get_args_and_format_string()`]: crate::format_args::get_args_and_format_string()"] # [derive (Debug , Clone)] pub struct SpanError { pub err : Error , pub span : Option < Span > , }
    };
}

SpanError!()
macro_rules! Read {
    () => {
        # [doc = " A structure passing every [`read`](std::io::Read::read()) call through to the contained Progress instance using [`inc_by(bytes_read)`](Count::inc_by())."] pub struct Read < T , P > { # [doc = " The implementor of [`std::io::Read`] to which progress is added"] pub inner : T , # [doc = " The progress instance receiving progress information on each invocation of `reader`"] pub progress : P , }
    };
}

Read!();
macro_rules! deps {
    () => {
        LineEnding!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl Default for LineEnding { # [cfg (windows)] fn default () -> LineEnding { LineEnding :: CRLF } # [cfg (not (windows))] fn default () -> LineEnding { LineEnding :: LF } }
    };
}

impl_61!();
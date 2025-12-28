macro_rules! macro_42 {
    () => {
        rustc_index :: newtype_index ! { # [debug_format = "OutlivesConstraintIndex({})"] pub (crate) struct OutlivesConstraintIndex { } }
    };
}

macro_42!();
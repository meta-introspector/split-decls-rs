// Generated macro for StreamErrorInto (trait)
macro_rules! Depcrate_errorStreamErrorInto {
() => {
// Module: crate::error
// Provides: {"StreamErrorInto"}
// Dependencies: {}
# [doc = " Defines a conversion between two stream error types."] # [doc = ""] # [doc = " Like `StreamError::into_other` but with a more general signature"] # [doc = " (This will take the place of `into_other` on breaking release of combine)"] pub trait StreamErrorInto < Item , Range > : Sized { fn into_other_error < T , Item2 , Range2 > (self) -> T where T : StreamError < Item2 , Range2 > , Item2 : From < Item > , Range2 : From < Range > ; }
};
}

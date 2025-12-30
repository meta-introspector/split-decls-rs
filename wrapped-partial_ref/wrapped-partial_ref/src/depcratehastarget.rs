// Generated macro for HasTarget (trait)
macro_rules! DepcrateHasTarget {
() => {
// Module: crate
// Provides: {"HasTarget"}
// Dependencies: {}
# [doc = " Helper trait to associate the target type with a [`PartialRef`] without needing a lifetime."] pub trait HasTarget { # [doc = " The referenced type."] # [doc = ""] type Target : PartialRefTarget + ? Sized ; }
};
}

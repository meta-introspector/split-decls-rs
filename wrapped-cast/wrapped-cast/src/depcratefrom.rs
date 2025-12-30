// Generated macro for From (trait)
macro_rules! DepcrateFrom {
() => {
// Module: crate
// Provides: {"From"}
// Dependencies: {}
# [doc = " The \"cast from\" operation"] pub trait From < Src > { # [doc = " The result of the cast operation: either `Self` or `Result<Self, Error>`"] type Output ; # [doc = " Checked cast from `Src` to `Self`"] fn cast (_ : Src) -> Self :: Output ; }
};
}

// Generated macro for Print (struct)
macro_rules! Depcrate_stylePrint {
() => {
// Module: crate::style
// Provides: {"Print"}
// Dependencies: {}
# [doc = " A command that prints the given displayable type."] # [doc = ""] # [doc = " Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Print < T : Display > (pub T) ;
};
}

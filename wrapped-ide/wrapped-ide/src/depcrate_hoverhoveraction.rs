// Generated macro for HoverAction (enum)
macro_rules! Depcrate_hoverHoverAction {
() => {
// Module: crate::hover
// Provides: {"HoverAction"}
// Dependencies: {}
# [derive (Debug , Clone , Hash , PartialEq , Eq , UpmapFromRaFixture)] pub enum HoverAction { Runnable (Runnable) , Implementation (FilePosition) , Reference (FilePosition) , GoToType (Vec < HoverGotoTypeData >) , }
};
}

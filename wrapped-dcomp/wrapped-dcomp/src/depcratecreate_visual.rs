// Generated macro for create_visual (function)
macro_rules! Depcratecreate_visual {
() => {
// Module: crate
// Provides: {"create_visual"}
// Dependencies: {}
fn create_visual (device : & IDCompositionDesktopDevice) -> Result < IDCompositionVisual2 > { unsafe { let visual = device . CreateVisual () ? ; visual . SetBackFaceVisibility (DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN) ? ; Ok (visual) } }
};
}

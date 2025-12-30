// Generated macro for CoordTranslate (trait)
macro_rules! Depcrate_coord_translateCoordTranslate {
() => {
// Module: crate::coord::translate
// Provides: {"CoordTranslate"}
// Dependencies: {}
# [doc = " The trait that translates some customized object to the backend coordinate"] pub trait CoordTranslate { # [doc = " Specifies the object to be translated from"] type From ; # [doc = " Translate the guest coordinate to the guest coordinate"] fn translate (& self , from : & Self :: From) -> BackendCoord ; # [doc = " Get the Z-value of current coordinate"] fn depth (& self , _from : & Self :: From) -> i32 { 0 } }
};
}

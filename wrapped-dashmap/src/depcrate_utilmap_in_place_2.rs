// Generated macro for map_in_place_2 (function)
macro_rules! Depcrate_utilmap_in_place_2 {
() => {
// Module: crate::util
// Provides: {"map_in_place_2"}
// Dependencies: {}
pub fn map_in_place_2 < T , U , F : FnOnce (U , T) -> T > ((k , v) : (U , & mut T) , f : F) { unsafe { let promote_panic_to_abort = AbortOnPanic ; ptr :: write (v , f (k , ptr :: read (v))) ; std :: mem :: forget (promote_panic_to_abort) ; } }
};
}

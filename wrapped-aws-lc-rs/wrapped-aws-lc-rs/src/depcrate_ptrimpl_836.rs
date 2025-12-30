// Generated macro for impl_836 (impl)
macro_rules! Depcrate_ptrimpl_836 {
() => {
// Module: crate::ptr
// Provides: {"impl_836"}
// Dependencies: {}
impl < P : Pointer > DetachablePointer < P > { # [inline] pub fn new < T : IntoPointer < P > > (value : T) -> Result < Self , () > { if let Some (pointer) = value . into_pointer () { Ok (Self { pointer : Some (pointer) , }) } else { Err (()) } } # [inline] pub fn detach (mut self) -> P { self . pointer . take () . unwrap () } }
};
}

// Generated macro for impl_829 (impl)
macro_rules! Depcrate_ptrimpl_829 {
() => {
// Module: crate::ptr
// Provides: {"impl_829"}
// Dependencies: {}
impl < P : Pointer > ManagedPointer < P > { # [inline] pub fn new < T : IntoPointer < P > > (value : T) -> Result < Self , () > { if let Some (pointer) = value . into_pointer () { Ok (Self { pointer }) } else { Err (()) } } pub unsafe fn as_slice (& self , len : usize) -> & [P :: T] { core :: slice :: from_raw_parts (self . pointer . as_const_ptr () , len) } }
};
}

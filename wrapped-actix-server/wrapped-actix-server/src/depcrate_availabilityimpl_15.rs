// Generated macro for impl_15 (impl)
macro_rules! Depcrate_availabilityimpl_15 {
() => {
// Module: crate::availability
// Provides: {"impl_15"}
// Dependencies: {}
impl Availability { # [doc = " Check if any worker handle is available"] # [inline (always)] pub (crate) fn available (& self) -> bool { self . 0 . iter () . any (| a | * a != 0) } # [doc = " Check if worker handle is available by index"] # [inline (always)] pub (crate) fn get_available (& self , idx : usize) -> bool { let (offset , idx) = Self :: offset (idx) ; self . 0 [offset] & (1 << idx as u128) != 0 } # [doc = " Set worker handle available state by index."] pub (crate) fn set_available (& mut self , idx : usize , avail : bool) { let (offset , idx) = Self :: offset (idx) ; let off = 1 << idx as u128 ; if avail { self . 0 [offset] |= off ; } else { self . 0 [offset] &= ! off } } # [doc = " Set all worker handle to available state."] # [doc = " This would result in a re-check on all workers' availability."] pub (crate) fn set_available_all (& mut self , handles : & [WorkerHandleAccept]) { handles . iter () . for_each (| handle | { self . set_available (handle . idx () , true) ; }) } # [doc = " Get offset and adjusted index of given worker handle index."] pub (crate) fn offset (idx : usize) -> (usize , usize) { if idx < 128 { (0 , idx) } else if idx < 128 * 2 { (1 , idx - 128) } else if idx < 128 * 3 { (2 , idx - 128 * 2) } else if idx < 128 * 4 { (3 , idx - 128 * 3) } else { panic ! ("Max WorkerHandle count is 512") } } }
};
}

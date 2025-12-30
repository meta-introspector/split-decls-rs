// Generated macro for impl_120 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_120 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_120"}
// Dependencies: {}
impl < K , V > Node < K , V > { # [inline] unsafe fn put_entry (& mut self , entry : (K , V)) { self . entry . as_mut_ptr () . write (entry) } # [inline] unsafe fn entry_ref (& self) -> & (K , V) { & * self . entry . as_ptr () } # [inline] unsafe fn key_ref (& self) -> & K { & (* self . entry . as_ptr ()) . 0 } # [inline] unsafe fn entry_mut (& mut self) -> & mut (K , V) { & mut * self . entry . as_mut_ptr () } # [inline] unsafe fn take_entry (& mut self) -> (K , V) { self . entry . as_ptr () . read () } }
};
}

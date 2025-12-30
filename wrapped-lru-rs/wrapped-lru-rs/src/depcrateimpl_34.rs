// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < K , V , S > Drop for LruCache < K , V , S > { fn drop (& mut self) { self . map . drain () . for_each (| (_ , node) | unsafe { let mut node = * Box :: from_raw (node . as_ptr ()) ; ptr :: drop_in_place ((node) . key . as_mut_ptr ()) ; ptr :: drop_in_place ((node) . val . as_mut_ptr ()) ; }) ; let _head = unsafe { * Box :: from_raw (self . head) } ; let _tail = unsafe { * Box :: from_raw (self . tail) } ; } }
};
}

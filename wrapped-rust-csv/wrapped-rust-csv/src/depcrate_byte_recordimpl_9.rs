// Generated macro for impl_9 (impl)
macro_rules! Depcrate_byte_recordimpl_9 {
() => {
// Module: crate::byte_record
// Provides: {"impl_9"}
// Dependencies: {}
impl PartialEq for ByteRecord { fn eq (& self , other : & ByteRecord) -> bool { if self . len () != other . len () { return false ; } self . iter () . zip (other . iter ()) . all (| e | e . 0 == e . 1) } }
};
}

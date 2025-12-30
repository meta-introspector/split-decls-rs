// Generated macro for DelayLoadDescriptorIterator (struct)
macro_rules! Depcrate_read_pe_importDelayLoadDescriptorIterator {
() => {
// Module: crate::read::pe::import
// Provides: {"DelayLoadDescriptorIterator"}
// Dependencies: {}
# [doc = " A fallible iterator for the descriptors in the delay-load data directory."] # [derive (Debug , Clone)] pub struct DelayLoadDescriptorIterator < 'data > { data : Bytes < 'data > , null : bool , }
};
}

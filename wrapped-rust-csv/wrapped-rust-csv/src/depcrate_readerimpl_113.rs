// Generated macro for impl_113 (impl)
macro_rules! Depcrate_readerimpl_113 {
() => {
// Module: crate::reader
// Provides: {"impl_113"}
// Dependencies: {}
impl Default for ReaderBuilder { fn default () -> ReaderBuilder { ReaderBuilder { capacity : 8 * (1 << 10) , flexible : false , has_headers : true , trim : Trim :: default () , builder : Box :: new (CoreReaderBuilder :: default ()) , } } }
};
}

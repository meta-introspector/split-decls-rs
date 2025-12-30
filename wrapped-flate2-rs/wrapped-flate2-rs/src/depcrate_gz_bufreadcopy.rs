// Generated macro for copy (function)
macro_rules! Depcrate_gz_bufreadcopy {
() => {
// Module: crate::gz::bufread
// Provides: {"copy"}
// Dependencies: {}
fn copy (into : & mut [u8] , from : & [u8] , pos : & mut usize) -> usize { let min = cmp :: min (into . len () , from . len () - * pos) ; into [.. min] . copy_from_slice (& from [* pos .. * pos + min]) ; * pos += min ; min }
};
}

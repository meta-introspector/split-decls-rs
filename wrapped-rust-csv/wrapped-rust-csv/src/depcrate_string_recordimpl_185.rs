// Generated macro for impl_185 (impl)
macro_rules! Depcrate_string_recordimpl_185 {
() => {
// Module: crate::string_record
// Provides: {"impl_185"}
// Dependencies: {}
impl ops :: Index < usize > for StringRecord { type Output = str ; # [inline] fn index (& self , i : usize) -> & str { self . get (i) . unwrap () } }
};
}

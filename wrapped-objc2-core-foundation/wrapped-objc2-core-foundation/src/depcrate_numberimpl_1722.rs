// Generated macro for impl_1722 (impl)
macro_rules! Depcrate_numberimpl_1722 {
() => {
// Module: crate::number
// Provides: {"impl_1722"}
// Dependencies: {}
impl CFBoolean { pub fn new (value : bool) -> & 'static CFBoolean { if value { unsafe { kCFBooleanTrue } . unwrap () } else { unsafe { kCFBooleanFalse } . unwrap () } } pub fn as_bool (& self) -> bool { self . value () } }
};
}

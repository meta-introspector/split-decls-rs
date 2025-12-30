// Generated macro for property_callback (function)
macro_rules! Depcrateproperty_callback {
() => {
// Module: crate
// Provides: {"property_callback"}
// Dependencies: {}
unsafe fn property_callback (payload : * mut String , _name : * const c_char , value : * const c_char , _serial : u32) { let cvalue = CStr :: from_ptr (value) ; (* payload) = cvalue . to_str () . unwrap () . to_string () ; }
};
}

// Generated macro for Ptr (struct)
macro_rules! DepcratePtr {
() => {
// Module: crate
// Provides: {"Ptr"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone)] struct Ptr { # [doc = " if len <= 2, it's the string itself,"] # [doc = " otherwise it's an u16 offset into MAPPING"] chr : [u8 ; 2] , len : u8 , }
};
}

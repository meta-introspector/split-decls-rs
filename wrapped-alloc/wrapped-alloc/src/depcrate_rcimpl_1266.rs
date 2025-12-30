// Generated macro for impl_1266 (impl)
macro_rules! Depcrate_rcimpl_1266 {
() => {
// Module: crate::rc
// Provides: {"impl_1266"}
// Dependencies: {}
# [stable (feature = "shared_from_str" , since = "1.62.0")] impl From < Rc < str > > for Rc < [u8] > { # [doc = " Converts a reference-counted string slice into a byte slice."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let string: Rc<str> = Rc::from(\"eggplant\");"] # [doc = " let bytes: Rc<[u8]> = Rc::from(string);"] # [doc = " assert_eq!(\"eggplant\".as_bytes(), bytes.as_ref());"] # [doc = " ```"] # [inline] fn from (rc : Rc < str >) -> Self { unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const [u8]) } } }
};
}

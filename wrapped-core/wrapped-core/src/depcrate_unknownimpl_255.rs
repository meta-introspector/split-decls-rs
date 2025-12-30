// Generated macro for impl_255 (impl)
macro_rules! Depcrate_unknownimpl_255 {
() => {
// Module: crate::unknown
// Provides: {"impl_255"}
// Dependencies: {}
impl PartialEq for IUnknown { fn eq (& self , other : & Self) -> bool { core :: ptr :: eq (self . as_raw () , other . as_raw ()) || self . cast :: < Self > () . unwrap () . 0 == other . cast :: < Self > () . unwrap () . 0 } }
};
}

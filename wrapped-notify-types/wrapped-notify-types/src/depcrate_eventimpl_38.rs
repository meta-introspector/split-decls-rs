// Generated macro for impl_38 (impl)
macro_rules! Depcrate_eventimpl_38 {
() => {
// Module: crate::event
// Provides: {"impl_38"}
// Dependencies: {}
impl PartialEq for Event { fn eq (& self , other : & Self) -> bool { self . kind . eq (& other . kind) && self . paths . eq (& other . paths) && self . tracker () . eq (& other . tracker ()) && self . flag () . eq (& other . flag ()) && self . info () . eq (& other . info ()) && self . source () . eq (& other . source ()) } }
};
}

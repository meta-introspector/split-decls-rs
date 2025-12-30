// Generated macro for impl_38 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_38 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_38"}
// Dependencies: {}
impl < Key , Value > Debug for OccupiedEntry < '_ , Key , Value > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("values" , & self . iter ()) . finish () } }
};
}

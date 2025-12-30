// Generated macro for impl_41 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_41 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_41"}
// Dependencies: {}
impl < Key , Value , State > Debug for VacantEntry < '_ , Key , Value , State > where Key : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_tuple ("VacantEntry") . field (& self . key) . finish () } }
};
}

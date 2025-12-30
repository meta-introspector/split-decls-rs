// Generated macro for impl_131 (impl)
macro_rules! Depcrate_unitimpl_131 {
() => {
// Module: crate::unit
// Provides: {"impl_131"}
// Dependencies: {}
impl < R : gimli :: Reader > DwoUnit < R > { fn unit_ref (& self) -> gimli :: UnitRef < '_ , R > { gimli :: UnitRef :: new (& self . sections , & self . dw_unit) } }
};
}

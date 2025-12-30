// Generated macro for impl_526 (impl)
macro_rules! Depcrate_read_lineimpl_526 {
() => {
// Module: crate::read::line
// Provides: {"impl_526"}
// Dependencies: {}
impl < R : Reader > LineInstructions < R > { fn remove_trailing (& self , other : & LineInstructions < R >) -> Result < LineInstructions < R > > { let offset = other . input . offset_from (& self . input) ; let mut input = self . input . clone () ; input . truncate (offset) ? ; Ok (LineInstructions { input }) } }
};
}

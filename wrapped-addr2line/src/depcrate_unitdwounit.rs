// Generated macro for DwoUnit (struct)
macro_rules! Depcrate_unitDwoUnit {
() => {
// Module: crate::unit
// Provides: {"DwoUnit"}
// Dependencies: {}
# [doc = " A DWO unit has its own DWARF sections."] struct DwoUnit < R : gimli :: Reader > { sections : Arc < gimli :: Dwarf < R > > , dw_unit : gimli :: Unit < R > , }
};
}

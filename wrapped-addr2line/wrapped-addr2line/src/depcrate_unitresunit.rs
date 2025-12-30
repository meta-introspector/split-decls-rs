// Generated macro for ResUnit (struct)
macro_rules! Depcrate_unitResUnit {
() => {
// Module: crate::unit
// Provides: {"ResUnit"}
// Dependencies: {}
pub (crate) struct ResUnit < R : gimli :: Reader > { offset : gimli :: DebugInfoOffset < R :: Offset > , dw_unit : gimli :: Unit < R > , pub (crate) lang : Option < gimli :: DwLang > , lines : LazyLines , functions : LazyFunctions < R > , dwo : LazyResult < Option < Box < DwoUnit < R > > > > , }
};
}

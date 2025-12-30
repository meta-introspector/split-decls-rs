// Generated macro for DebugInfoUnitHeadersIter (struct)
macro_rules! Depcrate_read_unitDebugInfoUnitHeadersIter {
() => {
// Module: crate::read::unit
// Provides: {"DebugInfoUnitHeadersIter"}
// Dependencies: {}
# [doc = " An iterator over the units of a .debug_info section."] # [doc = ""] # [doc = " See the [documentation on"] # [doc = " `DebugInfo::units`](./struct.DebugInfo.html#method.units) for more detail."] # [derive (Clone , Debug)] pub struct DebugInfoUnitHeadersIter < R : Reader > { input : R , offset : UnitSectionOffset < R :: Offset > , }
};
}

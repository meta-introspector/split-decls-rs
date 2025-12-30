// Generated macro for RangeIter (struct)
macro_rules! Depcrate_read_dwarfRangeIter {
() => {
// Module: crate::read::dwarf
// Provides: {"RangeIter"}
// Dependencies: {}
# [doc = " An iterator for the address ranges of a `DebuggingInformationEntry`."] # [doc = ""] # [doc = " Returned by `Dwarf::die_ranges` and `Dwarf::unit_ranges`."] # [derive (Debug)] pub struct RangeIter < R : Reader > (RangeIterInner < R >) ;
};
}

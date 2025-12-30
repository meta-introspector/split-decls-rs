// Generated macro for LocationRangeIter (struct)
macro_rules! Depcrate_unitLocationRangeIter {
() => {
// Module: crate::unit
// Provides: {"LocationRangeIter"}
// Dependencies: {}
# [doc = " Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`."] pub struct LocationRangeIter < 'ctx , R : gimli :: Reader > { unit_iter : Box < dyn Iterator < Item = (& 'ctx ResUnit < R > , & 'ctx gimli :: Range) > + 'ctx > , iter : Option < LineLocationRangeIter < 'ctx > > , probe_low : u64 , probe_high : u64 , sections : & 'ctx gimli :: Dwarf < R > , }
};
}

// Generated macro for impl_68 (impl)
macro_rules! Depcrate_lineimpl_68 {
() => {
// Module: crate::line
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'ctx > Iterator for LineLocationRangeIter < 'ctx > { type Item = (u64 , u64 , Location < 'ctx >) ; fn next (& mut self) -> Option < (u64 , u64 , Location < 'ctx >) > { while let Some (seq) = self . lines . sequences . get (self . seq_idx) { if seq . start >= self . probe_high { break ; } match seq . rows . get (self . row_idx) { Some (row) => { if row . address >= self . probe_high { break ; } let nextaddr = seq . rows . get (self . row_idx + 1) . map (| row | row . address) . unwrap_or (seq . end) ; let item = (row . address , nextaddr - row . address , self . lines . row_location (row) ,) ; self . row_idx += 1 ; return Some (item) ; } None => { self . seq_idx += 1 ; self . row_idx = 0 ; } } } None } }
};
}

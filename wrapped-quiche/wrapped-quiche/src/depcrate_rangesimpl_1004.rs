// Generated macro for impl_1004 (impl)
macro_rules! Depcrate_rangesimpl_1004 {
() => {
// Module: crate::ranges
// Provides: {"impl_1004"}
// Dependencies: {}
impl BTreeRangeSet { fn insert (& mut self , item : Range < u64 >) { let mut start = item . start ; let mut end = item . end ; if let Some (r) = self . prev_to (start) { if range_overlaps (& r , & item) { self . inner . remove (& r . start) ; start = cmp :: min (start , r . start) ; end = cmp :: max (end , r . end) ; } } while let Some (r) = self . next_to (start) { if item . contains (& r . start) && item . contains (& r . end) { self . inner . remove (& r . start) ; continue ; } if ! range_overlaps (& r , & item) { break ; } self . inner . remove (& r . start) ; start = cmp :: min (start , r . start) ; end = cmp :: max (end , r . end) ; } if self . inner . len () >= self . capacity { self . inner . pop_first () ; } self . inner . insert (start , end) ; } fn remove_until (& mut self , largest : u64) { let ranges : Vec < Range < u64 > > = self . inner . range ((Bound :: Unbounded , Bound :: Included (& largest))) . map (| (& s , & e) | s .. e) . collect () ; for r in ranges { self . inner . remove (& r . start) ; if r . end > largest + 1 { let start = largest + 1 ; self . insert (start .. r . end) ; } } } fn prev_to (& self , item : u64) -> Option < Range < u64 > > { self . inner . range ((Bound :: Unbounded , Bound :: Included (item))) . map (| (& s , & e) | s .. e) . next_back () } fn next_to (& self , item : u64) -> Option < Range < u64 > > { self . inner . range ((Bound :: Included (item) , Bound :: Unbounded)) . map (| (& s , & e) | s .. e) . next () } }
};
}

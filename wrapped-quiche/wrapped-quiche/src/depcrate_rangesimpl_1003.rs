// Generated macro for impl_1003 (impl)
macro_rules! Depcrate_rangesimpl_1003 {
() => {
// Module: crate::ranges
// Provides: {"impl_1003"}
// Dependencies: {}
impl InlineRangeSet { fn insert (& mut self , item : Range < u64 >) { let start = item . start ; let mut end = item . end ; let mut pos = 0 ; loop { match self . inner . get_mut (pos) { Some ((s , e)) => { if start > * e { pos += 1 ; continue ; } if end < * s { if self . inner . len () == self . capacity { self . inner . remove (0) ; pos -= 1 ; } self . inner . insert (pos , (start , end)) ; return ; } if start < * s { * s = start ; } if end > * e { * e = end ; break ; } else { return ; } } , None => { if self . inner . len () == self . capacity { self . inner . remove (0) ; } self . inner . push ((start , end)) ; return ; } , } } while let Some ((s , e)) = self . inner . get (pos + 1) . copied () { if end < s { break ; } let new_e = e . max (end) ; self . inner [pos] . 1 = new_e ; end = new_e ; self . inner . remove (pos + 1) ; } } fn remove_until (& mut self , largest : u64) { while let Some ((s , e)) = self . inner . first_mut () { if largest >= * e { self . inner . remove (0) ; continue ; } * s = (largest + 1) . max (* s) ; if * s == * e { self . inner . remove (0) ; } break ; } } }
};
}

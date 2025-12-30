// Generated macro for impl_117 (impl)
macro_rules! Depcrate_timezone_implimpl_117 {
() => {
// Module: crate::timezone_impl
// Provides: {"impl_117"}
// Dependencies: {}
impl FixedTimespanSet { fn len (& self) -> usize { 1 + self . rest . len () } fn utc_span (& self , index : usize) -> Span { debug_assert ! (index < self . len ()) ; Span { begin : if index == 0 { None } else { Some (self . rest [index - 1] . 0) } , end : if index == self . rest . len () { None } else { Some (self . rest [index] . 0) } , } } fn local_span (& self , index : usize) -> Span { debug_assert ! (index < self . len ()) ; Span { begin : if index == 0 { None } else { let span = self . rest [index - 1] ; Some (span . 0 + span . 1 . offset as i64) } , end : if index == self . rest . len () { None } else if index == 0 { Some (self . rest [index] . 0 + self . first . offset as i64) } else { Some (self . rest [index] . 0 + self . rest [index - 1] . 1 . offset as i64) } , } } fn get (& self , index : usize) -> FixedTimespan { debug_assert ! (index < self . len ()) ; if index == 0 { self . first } else { self . rest [index - 1] . 1 } } }
};
}

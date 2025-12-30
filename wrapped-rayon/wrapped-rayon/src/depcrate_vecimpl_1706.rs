// Generated macro for impl_1706 (impl)
macro_rules! Depcrate_vecimpl_1706 {
() => {
// Module: crate::vec
// Provides: {"impl_1706"}
// Dependencies: {}
impl < 'data , T : Send > Drop for Drain < 'data , T > { fn drop (& mut self) { let Range { start , end } = self . range ; if self . vec . len () == self . orig_len { self . vec . drain (start .. end) ; } else if start == end { unsafe { self . vec . set_len (self . orig_len) ; } } else if end < self . orig_len { unsafe { let ptr = self . vec . as_mut_ptr () . add (start) ; let tail_ptr = self . vec . as_ptr () . add (end) ; let tail_len = self . orig_len - end ; ptr :: copy (tail_ptr , ptr , tail_len) ; self . vec . set_len (start + tail_len) ; } } } }
};
}

// Generated macro for totalorder_impl (macro)
macro_rules! Depcrate_floattotalorder_impl {
() => {
// Module: crate::float
// Provides: {"totalorder_impl"}
// Dependencies: {}
macro_rules ! totalorder_impl { ($ T : ident , $ I : ident , $ U : ident , $ bits : expr) => { impl TotalOrder for $ T { # [inline] # [cfg (has_total_cmp)] fn total_cmp (& self , other : & Self) -> Ordering { Self :: total_cmp (& self , other) } # [inline] # [cfg (not (has_total_cmp))] fn total_cmp (& self , other : & Self) -> Ordering { let mut left = self . to_bits () as $ I ; let mut right = other . to_bits () as $ I ; left ^= (((left >> ($ bits - 1)) as $ U) >> 1) as $ I ; right ^= (((right >> ($ bits - 1)) as $ U) >> 1) as $ I ; left . cmp (& right) } } } ; }
};
}

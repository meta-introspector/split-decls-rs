// Generated macro for impl_ord_with_size_types (macro)
macro_rules! Depcrate_ordimpl_ord_with_size_types {
() => {
// Module: crate::ord
// Provides: {"impl_ord_with_size_types"}
// Dependencies: {}
macro_rules ! impl_ord_with_size_types { ($ ($ t : ty) *) => ($ (impl NumOrd <$ t > for usize { # [inline] fn num_partial_cmp (& self , other : &$ t) -> Option < Ordering > { # [cfg (target_pointer_width = "32")] { (* self as u32) . num_partial_cmp (other) } # [cfg (target_pointer_width = "64")] { (* self as u64) . num_partial_cmp (other) } } } impl NumOrd < usize > for $ t { # [inline] fn num_partial_cmp (& self , other : & usize) -> Option < Ordering > { # [cfg (target_pointer_width = "32")] { self . num_partial_cmp (& (* other as u32)) } # [cfg (target_pointer_width = "64")] { self . num_partial_cmp (& (* other as u64)) } } } impl NumOrd <$ t > for isize { # [inline] fn num_partial_cmp (& self , other : &$ t) -> Option < Ordering > { # [cfg (target_pointer_width = "32")] { (* self as i32) . num_partial_cmp (other) } # [cfg (target_pointer_width = "64")] { (* self as i64) . num_partial_cmp (other) } } } impl NumOrd < isize > for $ t { # [inline] fn num_partial_cmp (& self , other : & isize) -> Option < Ordering > { # [cfg (target_pointer_width = "32")] { self . num_partial_cmp (& (* other as i32)) } # [cfg (target_pointer_width = "64")] { self . num_partial_cmp (& (* other as i64)) } } }) *) ; }
};
}

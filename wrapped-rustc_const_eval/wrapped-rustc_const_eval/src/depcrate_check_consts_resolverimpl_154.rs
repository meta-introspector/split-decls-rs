// Generated macro for impl_154 (impl)
macro_rules! Depcrate_check_consts_resolverimpl_154 {
() => {
// Module: crate::check_consts::resolver
// Provides: {"impl_154"}
// Dependencies: {}
impl < C > DebugWithContext < C > for State { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("qualif: ") ? ; self . qualif . fmt_with (ctxt , f) ? ; f . write_str (" borrow: ") ? ; self . borrow . fmt_with (ctxt , f) ? ; Ok (()) } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self == old { return Ok (()) ; } if self . qualif != old . qualif { f . write_str ("qualif: ") ? ; self . qualif . fmt_diff_with (& old . qualif , ctxt , f) ? ; f . write_str ("\n") ? ; } if self . borrow != old . borrow { f . write_str ("borrow: ") ? ; self . qualif . fmt_diff_with (& old . borrow , ctxt , f) ? ; f . write_str ("\n") ? ; } Ok (()) } }
};
}

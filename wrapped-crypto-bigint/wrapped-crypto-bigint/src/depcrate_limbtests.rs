// Generated macro for tests (module)
macro_rules! Depcrate_limbtests {
() => {
// Module: crate::limb
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (feature = "alloc")] use { super :: Limb , alloc :: format } ; # [cfg (feature = "alloc")] # [test] fn debug () { # [cfg (target_pointer_width = "32")] assert_eq ! (format ! ("{:?}" , Limb (42)) , "Limb(0x0000002A)") ; # [cfg (target_pointer_width = "64")] assert_eq ! (format ! ("{:?}" , Limb (42)) , "Limb(0x000000000000002A)") ; } }
};
}

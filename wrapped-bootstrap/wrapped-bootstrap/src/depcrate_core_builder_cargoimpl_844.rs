// Generated macro for impl_844 (impl)
macro_rules! Depcrate_core_builder_cargoimpl_844 {
() => {
// Module: crate::core::builder::cargo
// Provides: {"impl_844"}
// Dependencies: {}
impl Rustflags { fn new (target : TargetSelection) -> Rustflags { let mut ret = Rustflags (String :: new () , target) ; ret . propagate_cargo_env ("RUSTFLAGS") ; ret } # [doc = " By default, cargo will pick up on various variables in the environment. However, bootstrap"] # [doc = " reuses those variables to pass additional flags to rustdoc, so by default they get"] # [doc = " overridden. Explicitly add back any previous value in the environment."] # [doc = ""] # [doc = " `prefix` is usually `RUSTFLAGS` or `RUSTDOCFLAGS`."] fn propagate_cargo_env (& mut self , prefix : & str) { self . env (prefix) ; let target_specific = format ! ("CARGO_TARGET_{}_{}" , crate :: envify (& self . 1 . triple) , prefix) ; self . env (& target_specific) ; } fn env (& mut self , env : & str) { if let Ok (s) = env :: var (env) { for part in s . split (' ') { self . arg (part) ; } } } fn arg (& mut self , arg : & str) -> & mut Self { assert_eq ! (arg . split (' ') . count () , 1) ; if ! self . 0 . is_empty () { self . 0 . push (' ') ; } self . 0 . push_str (arg) ; self } }
};
}

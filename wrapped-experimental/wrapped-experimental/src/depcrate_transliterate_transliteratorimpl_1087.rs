// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1087 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1087"}
// Dependencies: {}
impl SimpleId < '_ > { fn transliterate (& self , mut rep : Replaceable , env : & Env) { let inner = env . get (self . id . as_ref ()) . unwrap () ; rep . for_each_run (& self . filter , | run | { inner . transliterate (run . child () , env) }) } }
};
}

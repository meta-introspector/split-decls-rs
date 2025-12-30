// Generated macro for global (module)
macro_rules! Depcrate_seedglobal {
() => {
// Module: crate::seed
// Provides: {"global"}
// Dependencies: {}
# [cfg (not (target_has_atomic = "8"))] mod global { use super :: * ; # [derive (Copy , Clone , Debug)] pub struct GlobalSeed { } impl GlobalSeed { # [inline (always)] pub fn new () -> Self { Self { } } # [inline (always)] pub fn get (self) -> & 'static SharedSeed { & super :: FIXED_GLOBAL_SEED } } }
};
}

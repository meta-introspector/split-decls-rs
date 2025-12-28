macro_rules! deps {
    () => {
        SharedSeed!();
    };
}

macro_rules! global {
    () => {
        deps!();
        # [cfg (not (target_has_atomic = "8"))] mod global { use super :: * ; # [derive (Copy , Clone , Debug)] pub struct GlobalSeed { } impl GlobalSeed { # [inline (always)] pub fn new () -> Self { Self { } } # [inline (always)] pub fn get (self) -> & 'static SharedSeed { & super :: FIXED_GLOBAL_SEED } } }
    };
}

global!();
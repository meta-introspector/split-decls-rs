// Generated macro for create_factory (function)
macro_rules! Depcratecreate_factory {
() => {
// Module: crate
// Provides: {"create_factory"}
// Dependencies: {}
fn create_factory () -> Result < ID2D1Factory1 > { let mut options = D2D1_FACTORY_OPTIONS :: default () ; if cfg ! (debug_assertions) { options . debugLevel = D2D1_DEBUG_LEVEL_INFORMATION ; } unsafe { D2D1CreateFactory (D2D1_FACTORY_TYPE_SINGLE_THREADED , Some (& options)) } }
};
}

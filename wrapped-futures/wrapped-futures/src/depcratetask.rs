// Generated macro for task (module)
macro_rules! Depcratetask {
() => {
// Module: crate
// Provides: {"task"}
// Dependencies: {}
mod task { use cfg_if :: cfg_if ; cfg_if ! { if # [cfg (target_feature = "atomics")] { mod wait_async_polyfill ; mod multithread ; pub (crate) use multithread ::*; } else { mod singlethread ; pub (crate) use singlethread ::*; } } }
};
}

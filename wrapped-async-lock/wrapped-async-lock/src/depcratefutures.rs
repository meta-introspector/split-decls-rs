// Generated macro for futures (module)
macro_rules! Depcratefutures {
() => {
// Module: crate
// Provides: {"futures"}
// Dependencies: {}
pub mod futures { # ! [doc = " Named futures for use with `async_lock` primitives."] pub use crate :: barrier :: BarrierWait ; pub use crate :: mutex :: { Lock , LockArc } ; pub use crate :: rwlock :: futures :: { Read , ReadArc , UpgradableRead , UpgradableReadArc , Upgrade , UpgradeArc , Write , WriteArc , } ; pub use crate :: semaphore :: { Acquire , AcquireArc } ; }
};
}

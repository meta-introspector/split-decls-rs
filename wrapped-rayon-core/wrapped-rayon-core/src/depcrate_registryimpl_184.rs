// Generated macro for impl_184 (impl)
macro_rules! Depcrate_registryimpl_184 {
() => {
// Module: crate::registry
// Provides: {"impl_184"}
// Dependencies: {}
impl From < ThreadBuilder > for WorkerThread { fn from (thread : ThreadBuilder) -> Self { Self { worker : thread . worker , stealer : thread . stealer , fifo : JobFifo :: new () , index : thread . index , rng : XorShift64Star :: new () , registry : thread . registry , } } }
};
}

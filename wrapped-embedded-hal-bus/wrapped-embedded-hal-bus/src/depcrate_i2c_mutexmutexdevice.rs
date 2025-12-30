// Generated macro for MutexDevice (struct)
macro_rules! Depcrate_i2c_mutexMutexDevice {
() => {
// Module: crate::i2c::mutex
// Provides: {"MutexDevice"}
// Dependencies: {}
# [doc = " `std` `Mutex`-based shared bus [`I2c`] implementation."] # [doc = ""] # [doc = " Sharing is implemented with an `std` [`Mutex`]. It allows a single bus across multiple threads,"] # [doc = " with finer-grained locking than [`CriticalSectionDevice`](super::CriticalSectionDevice). The downside is that"] # [doc = " it is only available in `std` targets."] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub struct MutexDevice < 'a , T > { bus : & 'a Mutex < T > , }
};
}

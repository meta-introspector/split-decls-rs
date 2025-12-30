// Generated macro for MutexDevice (struct)
macro_rules! Depcrate_spi_mutexMutexDevice {
() => {
// Module: crate::spi::mutex
// Provides: {"MutexDevice"}
// Dependencies: {}
# [doc = " `std` `Mutex`-based shared bus [`SpiDevice`] implementation."] # [doc = ""] # [doc = " This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,"] # [doc = " each with its own `CS` pin."] # [doc = ""] # [doc = " Sharing is implemented with a `std` [`Mutex`]. It allows a single bus across multiple threads,"] # [doc = " with finer-grained locking than [`CriticalSectionDevice`](super::CriticalSectionDevice). The downside is"] # [doc = " it is only available in `std` targets."] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub struct MutexDevice < 'a , BUS , CS , D > { bus : & 'a Mutex < BUS > , cs : CS , delay : D , }
};
}

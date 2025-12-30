// Generated macro for RefCellDevice (struct)
macro_rules! Depcrate_spi_refcellRefCellDevice {
() => {
// Module: crate::spi::refcell
// Provides: {"RefCellDevice"}
// Dependencies: {}
# [doc = " `RefCell`-based shared bus [`SpiDevice`] implementation."] # [doc = ""] # [doc = " This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,"] # [doc = " each with its own `CS` pin."] # [doc = ""] # [doc = " Sharing is implemented with a `RefCell`. This means it has low overhead, but `RefCellDevice` instances are not `Send`,"] # [doc = " so it only allows sharing within a single thread (interrupt priority level). If you need to share a bus across several"] # [doc = " threads, use [`CriticalSectionDevice`](super::CriticalSectionDevice) instead."] pub struct RefCellDevice < 'a , BUS , CS , D > { bus : & 'a RefCell < BUS > , cs : CS , delay : D , }
};
}

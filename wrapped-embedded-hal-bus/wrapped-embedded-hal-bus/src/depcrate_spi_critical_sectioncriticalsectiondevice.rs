// Generated macro for CriticalSectionDevice (struct)
macro_rules! Depcrate_spi_critical_sectionCriticalSectionDevice {
() => {
// Module: crate::spi::critical_section
// Provides: {"CriticalSectionDevice"}
// Dependencies: {}
# [doc = " `critical-section`-based shared bus [`SpiDevice`] implementation."] # [doc = ""] # [doc = " This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,"] # [doc = " each with its own `CS` pin."] # [doc = ""] # [doc = " Sharing is implemented with a `critical-section` [`Mutex`]. A critical section is taken for"] # [doc = " the entire duration of a transaction. This allows sharing a single bus across multiple threads (interrupt priority levels)."] # [doc = " The downside is critical sections typically require globally disabling interrupts, so `CriticalSectionDevice` will likely"] # [doc = " negatively impact real-time properties, such as interrupt latency. If you can, prefer using"] # [doc = " [`RefCellDevice`](super::RefCellDevice) instead, which does not require taking critical sections."] pub struct CriticalSectionDevice < 'a , BUS , CS , D > { bus : & 'a Mutex < RefCell < BUS > > , cs : CS , delay : D , }
};
}

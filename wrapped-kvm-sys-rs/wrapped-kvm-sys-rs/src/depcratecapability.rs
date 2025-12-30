// Generated macro for Capability (enum)
macro_rules! DepcrateCapability {
() => {
// Module: crate
// Provides: {"Capability"}
// Dependencies: {}
# [doc = " KVM system capabilities"] # [allow (missing_docs)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [repr (i32)] pub enum Capability { Irqchip , Hlt , MmuShadowCacheControl , UserMemory , SetTssAddr , Vapic = 6 , ExtCpuid , ClockSource , NrVcpus , NrMemSlots , Pit , NopIoDelay , PvMmu , MpState , CoalescedMmio , SyncMmu , IoMmu = 18 , DestroyMemoryRegionWorks = 21 , UserNmi , MaxVcpus = 66 , CheckExtensionVm = 105 , }
};
}

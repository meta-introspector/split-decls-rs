// Generated macro for impl_192 (impl)
macro_rules! Depcrate_generatedimpl_192 {
() => {
// Module: crate::generated
// Provides: {"impl_192"}
// Dependencies: {}
impl SRDeletionReason { # [doc = " The user initiated deletion"] # [doc (alias = "SRDeletionReasonUserInitiated")] pub const UserInitiated : Self = Self (0) ; # [doc = " Samples were removed due to low disk conditions"] # [doc (alias = "SRDeletionReasonLowDiskSpace")] pub const LowDiskSpace : Self = Self (1) ; # [doc = " Samples were removed because they were recorded beyond our retention limit"] # [doc (alias = "SRDeletionReasonAgeLimit")] pub const AgeLimit : Self = Self (2) ; # [doc = " Samples were removed because there are no longer any interested clients"] # [doc (alias = "SRDeletionReasonNoInterestedClients")] pub const NoInterestedClients : Self = Self (3) ; # [doc = " Samples were removed because the system was in an invalid state"] # [doc (alias = "SRDeletionReasonSystemInitiated")] pub const SystemInitiated : Self = Self (4) ; }
};
}

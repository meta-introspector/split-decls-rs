// Generated macro for Exit (enum)
macro_rules! DepcrateExit {
() => {
// Module: crate
// Provides: {"Exit"}
// Dependencies: {}
# [doc = " KVM `run` exit reasons"] # [allow (missing_docs)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [repr (u32)] pub enum Exit { Unknown , Exception , Io , Hypercall , Debug , Hlt , Mmio , IrqWindowOpen , Shutdown , FailEntry , Intr , SetTpr , TprAccess , S390Sieic , S390Reset , Dcr , Nmi , InternalError , Osi , PaprHcall , S390Ucontrol , Watchdog , S390Tsch , Epr , SystemEvent , }
};
}

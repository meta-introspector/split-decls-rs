// Generated macro for MachineCallback (trait)
macro_rules! Depcrate_machineMachineCallback {
() => {
// Module: crate::machine
// Provides: {"MachineCallback"}
// Dependencies: {}
# [doc = " Trait for callbacks handling asynchronous machine operations."] pub trait MachineCallback < 'tcx , T > : VisitProvenance { # [doc = " The function to be invoked when the callback is fired."] fn call (self : Box < Self > , ecx : & mut InterpCx < 'tcx , MiriMachine < 'tcx > > , arg : T ,) -> InterpResult < 'tcx > ; }
};
}

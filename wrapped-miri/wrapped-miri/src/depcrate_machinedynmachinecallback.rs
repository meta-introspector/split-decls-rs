// Generated macro for DynMachineCallback (type)
macro_rules! Depcrate_machineDynMachineCallback {
() => {
// Module: crate::machine
// Provides: {"DynMachineCallback"}
// Dependencies: {}
# [doc = " Type alias for boxed machine callbacks with generic argument type."] pub type DynMachineCallback < 'tcx , T > = Box < dyn MachineCallback < 'tcx , T > + 'tcx > ;
};
}

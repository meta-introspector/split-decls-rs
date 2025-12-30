// Generated macro for MachLabelTrap (struct)
macro_rules! Depcrate_machinst_bufferMachLabelTrap {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachLabelTrap"}
// Dependencies: {}
# [doc = " A trap that is deferred to the next time an island is emitted for either"] # [doc = " traps, constants, or fixups."] struct MachLabelTrap { # [doc = " This label will refer to the trap's offset."] label : MachLabel , # [doc = " The code associated with this trap."] code : TrapCode , # [doc = " An optional source location to assign for this trap."] loc : Option < RelSourceLoc > , }
};
}

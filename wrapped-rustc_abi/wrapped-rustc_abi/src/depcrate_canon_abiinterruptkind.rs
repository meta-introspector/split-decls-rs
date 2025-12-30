// Generated macro for InterruptKind (enum)
macro_rules! Depcrate_canon_abiInterruptKind {
() => {
// Module: crate::canon_abi
// Provides: {"InterruptKind"}
// Dependencies: {}
# [doc = " Callee codegen for interrupts"] # [doc = ""] # [doc = " This is named differently from the \"Call\" enums because it is different:"] # [doc = " these \"ABI\" differences are not relevant to callers, since there is \"no caller\"."] # [doc = " These only affect callee codegen. making their categorization as distinct ABIs a bit peculiar."] # [derive (Copy , Clone , Debug)] # [derive (PartialOrd , Ord , PartialEq , Eq , Hash)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum InterruptKind { Avr , AvrNonBlocking , Msp430 , RiscvMachine , RiscvSupervisor , X86 , }
};
}

// Generated macro for ABIArgSlot (enum)
macro_rules! Depcrate_machinst_abiABIArgSlot {
() => {
// Module: crate::machinst::abi
// Provides: {"ABIArgSlot"}
// Dependencies: {}
# [doc = " A location for (part of) an argument or return value. These \"storage slots\""] # [doc = " are specified for each register-sized part of an argument."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ABIArgSlot { # [doc = " In a real register."] Reg { # [doc = " Register that holds this arg."] reg : RealReg , # [doc = " Value type of this arg."] ty : ir :: Type , # [doc = " Should this arg be zero- or sign-extended?"] extension : ir :: ArgumentExtension , } , # [doc = " Arguments only: on stack, at given offset from SP at entry."] Stack { # [doc = " Offset of this arg relative to the base of stack args."] offset : i64 , # [doc = " Value type of this arg."] ty : ir :: Type , # [doc = " Should this arg be zero- or sign-extended?"] extension : ir :: ArgumentExtension , } , }
};
}

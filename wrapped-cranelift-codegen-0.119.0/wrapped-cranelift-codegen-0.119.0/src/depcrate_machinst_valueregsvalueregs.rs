// Generated macro for ValueRegs (struct)
macro_rules! Depcrate_machinst_valueregsValueRegs {
() => {
// Module: crate::machinst::valueregs
// Provides: {"ValueRegs"}
// Dependencies: {}
# [doc = " Location at which a `Value` is stored in register(s): the value is located"] # [doc = " in one or more registers, depending on its width. A value may be stored in"] # [doc = " more than one register if the machine has no registers wide enough"] # [doc = " otherwise: for example, on a 32-bit architecture, we may store `I64` values"] # [doc = " in two registers, and `I128` values in four."] # [doc = ""] # [doc = " By convention, the register parts are kept in machine-endian order here."] # [doc = ""] # [doc = " N.B.: we cap the capacity of this at four (when any 32-bit target is"] # [doc = " enabled) or two (otherwise), and we use special in-band sentinel `Reg`"] # [doc = " values (`Reg::invalid()`) to avoid the need to carry a separate length. This"] # [doc = " allows the struct to be `Copy` (no heap or drop overhead) and be only 16 or"] # [doc = " 8 bytes, which is important for compiler performance."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct ValueRegs < R : Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel > { parts : [R ; VALUE_REGS_PARTS] , }
};
}

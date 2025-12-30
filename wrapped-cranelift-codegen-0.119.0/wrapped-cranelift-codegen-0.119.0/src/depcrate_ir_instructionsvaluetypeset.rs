// Generated macro for ValueTypeSet (struct)
macro_rules! Depcrate_ir_instructionsValueTypeSet {
() => {
// Module: crate::ir::instructions
// Provides: {"ValueTypeSet"}
// Dependencies: {}
# [doc = " A value type set describes the permitted set of types for a type variable."] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub struct ValueTypeSet { # [doc = " Allowed lane sizes"] pub lanes : BitSet16 , # [doc = " Allowed int widths"] pub ints : BitSet8 , # [doc = " Allowed float widths"] pub floats : BitSet8 , # [doc = " Allowed dynamic vectors minimum lane sizes"] pub dynamic_lanes : BitSet16 , }
};
}

// Generated macro for IntCC (enum)
macro_rules! Depcrate_ir_condcodesIntCC {
() => {
// Module: crate::ir::condcodes
// Provides: {"IntCC"}
// Dependencies: {}
# [doc = " Condition code for comparing integers."] # [doc = ""] # [doc = " This condition code is used by the `icmp` instruction to compare integer values. There are"] # [doc = " separate codes for comparing the integers as signed or unsigned numbers where it makes a"] # [doc = " difference."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum IntCC { # [doc = " `==`."] Equal , # [doc = " `!=`."] NotEqual , # [doc = " Signed `<`."] SignedLessThan , # [doc = " Signed `>=`."] SignedGreaterThanOrEqual , # [doc = " Signed `>`."] SignedGreaterThan , # [doc = " Signed `<=`."] SignedLessThanOrEqual , # [doc = " Unsigned `<`."] UnsignedLessThan , # [doc = " Unsigned `>=`."] UnsignedGreaterThanOrEqual , # [doc = " Unsigned `>`."] UnsignedGreaterThan , # [doc = " Unsigned `<=`."] UnsignedLessThanOrEqual , }
};
}

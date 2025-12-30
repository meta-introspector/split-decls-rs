// Generated macro for Loss (enum)
macro_rules! Depcrate_ieeeLoss {
() => {
// Module: crate::ieee
// Provides: {"Loss"}
// Dependencies: {}
# [doc = " Enum that represents what fraction of the LSB truncated bits of an fp number"] # [doc = " represent."] # [doc = ""] # [doc = " This essentially combines the roles of guard and sticky bits."] # [must_use] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum Loss { ExactlyZero , LessThanHalf , ExactlyHalf , MoreThanHalf , }
};
}

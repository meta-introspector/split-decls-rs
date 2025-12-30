// Generated macro for FieldBytes (type)
macro_rules! DepcrateFieldBytes {
() => {
// Module: crate
// Provides: {"FieldBytes"}
// Dependencies: {}
# [doc = " NIST P-521 field element serialized as bytes."] # [doc = ""] # [doc = " Byte array containing a serialized field element value (base field or"] # [doc = " scalar)."] pub type FieldBytes = elliptic_curve :: FieldBytes < NistP521 > ;
};
}

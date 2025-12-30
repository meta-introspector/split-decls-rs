// Generated macro for TlsAttr (enum)
macro_rules! DepcrateTlsAttr {
() => {
// Module: crate
// Provides: {"TlsAttr"}
// Dependencies: {}
# [doc = " Attributes supported by derive-macros in this crate"] # [derive (Clone)] enum TlsAttr { # [doc = " Prefix for custom serialization functions"] With (ExprPath) , # [doc = " Custom literal discriminant for an enum variant"] Discriminant (DiscriminantValue) , # [doc = " Skip this attribute during (de)serialization."] # [doc = ""] # [doc = " Note: The type of the attribute needs to implement [Default]."] # [doc = "       This is required to populate the field with a known"] # [doc = "       value during deserialization."] Skip , # [cfg (feature = "conditional_deserialization")] CdField , }
};
}

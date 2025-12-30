// Generated macro for TypeInfo (struct)
macro_rules! Depcrate_semanticsTypeInfo {
() => {
// Module: crate::semantics
// Provides: {"TypeInfo"}
// Dependencies: {}
# [derive (Debug)] pub struct TypeInfo < 'db > { # [doc = " The original type of the expression or pattern."] pub original : Type < 'db > , # [doc = " The adjusted type, if an adjustment happened."] pub adjusted : Option < Type < 'db > > , }
};
}

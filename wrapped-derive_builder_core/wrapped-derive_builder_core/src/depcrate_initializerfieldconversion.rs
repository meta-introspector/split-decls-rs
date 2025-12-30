// Generated macro for FieldConversion (enum)
macro_rules! Depcrate_initializerFieldConversion {
() => {
// Module: crate::initializer
// Provides: {"FieldConversion"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum FieldConversion < 'a > { # [doc = " Usual conversion: unwrap the Option from the builder, or (hope to) use a default value"] OptionOrDefault , # [doc = " Custom conversion is a block contents expression"] Block (& 'a BlockContents) , # [doc = " Custom conversion is just to move the field from the builder"] Move , }
};
}

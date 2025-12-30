// Generated macro for ConstCtOption (struct)
macro_rules! Depcrate_const_choiceConstCtOption {
() => {
// Module: crate::const_choice
// Provides: {"ConstCtOption"}
// Dependencies: {}
# [doc = " An equivalent of `subtle::CtOption` usable in a `const fn` context."] # [derive (Debug , Clone)] pub struct ConstCtOption < T > { value : T , is_some : ConstChoice , }
};
}

// Generated macro for call (function)
macro_rules! Depcrate_abicall {
() => {
// Module: crate::abi
// Provides: {"call"}
// Dependencies: {}
# [doc = " Generates an abstract sequence of instructions which represents this"] # [doc = " function being adapted as an imported function."] # [doc = ""] # [doc = " The instructions here, when executed, will emulate a language with"] # [doc = " interface types calling the concrete wasm implementation. The parameters"] # [doc = " for the returned instruction sequence are the language's own"] # [doc = " interface-types parameters. One instruction in the instruction stream"] # [doc = " will be a `Call` which represents calling the actual raw wasm function"] # [doc = " signature."] # [doc = ""] # [doc = " This function is useful, for example, if you're building a language"] # [doc = " generator for WASI bindings. This will document how to translate"] # [doc = " language-specific values into the wasm types to call a WASI function,"] # [doc = " and it will also automatically convert the results of the WASI function"] # [doc = " back to a language-specific value."] pub fn call (resolve : & Resolve , variant : AbiVariant , lift_lower : LiftLower , func : & Function , bindgen : & mut impl Bindgen , async_ : bool ,) { Generator :: new (resolve , bindgen) . call (func , variant , lift_lower , async_) ; }
};
}

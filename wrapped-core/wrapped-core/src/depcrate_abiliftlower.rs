// Generated macro for LiftLower (enum)
macro_rules! Depcrate_abiLiftLower {
() => {
// Module: crate::abi
// Provides: {"LiftLower"}
// Dependencies: {}
# [doc = " Whether the glue code surrounding a call is lifting arguments and lowering"] # [doc = " results or vice versa."] # [derive (Clone , Copy , PartialEq , Eq)] pub enum LiftLower { # [doc = " When the glue code lifts arguments and lowers results."] # [doc = ""] # [doc = " ```text"] # [doc = " Wasm --lift-args--> SourceLanguage; call; SourceLanguage --lower-results--> Wasm"] # [doc = " ```"] LiftArgsLowerResults , # [doc = " When the glue code lowers arguments and lifts results."] # [doc = ""] # [doc = " ```text"] # [doc = " SourceLanguage --lower-args--> Wasm; call; Wasm --lift-results--> SourceLanguage"] # [doc = " ```"] LowerArgsLiftResults , }
};
}

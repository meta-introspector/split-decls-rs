// Generated macro for Funclet (struct)
macro_rules! Depcrate_commonFunclet {
() => {
// Module: crate::common
// Provides: {"Funclet"}
// Dependencies: {}
# [doc = " A structure representing an active landing pad for the duration of a basic"] # [doc = " block."] # [doc = ""] # [doc = " Each `Block` may contain an instance of this, indicating whether the block"] # [doc = " is part of a landing pad or not. This is used to make decision about whether"] # [doc = " to emit `invoke` instructions (e.g., in a landing pad we don't continue to"] # [doc = " use `invoke`) and also about various function call metadata."] # [doc = ""] # [doc = " For GNU exceptions (`landingpad` + `resume` instructions) this structure is"] # [doc = " just a bunch of `None` instances (not too interesting), but for MSVC"] # [doc = " exceptions (`cleanuppad` + `cleanupret` instructions) this contains data."] # [doc = " When inside of a landing pad, each function call in LLVM IR needs to be"] # [doc = " annotated with which landing pad it's a part of. This is accomplished via"] # [doc = " the `OperandBundleDef` value created for MSVC landing pads."] pub (crate) struct Funclet < 'll > { cleanuppad : & 'll Value , operand : llvm :: OperandBundleBox < 'll > , }
};
}

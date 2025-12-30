// Generated macro for TargetFrontendConfig (struct)
macro_rules! Depcrate_isaTargetFrontendConfig {
() => {
// Module: crate::isa
// Provides: {"TargetFrontendConfig"}
// Dependencies: {}
# [doc = " This struct provides information that a frontend may need to know about a target to"] # [doc = " produce Cranelift IR for the target."] # [derive (Clone , Copy , Hash)] pub struct TargetFrontendConfig { # [doc = " The default calling convention of the target."] pub default_call_conv : CallConv , # [doc = " The pointer width of the target."] pub pointer_width : PointerWidth , # [doc = " The log2 of the target's page size and alignment."] # [doc = ""] # [doc = " Note that this may be an upper-bound that is larger than necessary for"] # [doc = " some platforms since it may depend on runtime configuration."] pub page_size_align_log2 : u8 , }
};
}

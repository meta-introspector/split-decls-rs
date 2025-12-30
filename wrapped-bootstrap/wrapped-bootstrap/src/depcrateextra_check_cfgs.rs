// Generated macro for EXTRA_CHECK_CFGS (const)
macro_rules! DepcrateEXTRA_CHECK_CFGS {
() => {
// Module: crate
// Provides: {"EXTRA_CHECK_CFGS"}
// Dependencies: {}
# [doc = " Extra `--check-cfg` to add when building the compiler or tools"] # [doc = " (Mode restriction, config name, config values (if any))"] # [expect (clippy :: type_complexity)] const EXTRA_CHECK_CFGS : & [(Option < Mode > , & str , Option < & [& 'static str] >)] = & [(Some (Mode :: Rustc) , "bootstrap" , None) , (Some (Mode :: Codegen) , "bootstrap" , None) , (Some (Mode :: ToolRustcPrivate) , "bootstrap" , None) , (Some (Mode :: ToolStd) , "bootstrap" , None) , (Some (Mode :: Rustc) , "llvm_enzyme" , None) , (Some (Mode :: Codegen) , "llvm_enzyme" , None) , (Some (Mode :: ToolRustcPrivate) , "llvm_enzyme" , None) , (Some (Mode :: ToolRustcPrivate) , "rust_analyzer" , None) , (Some (Mode :: ToolStd) , "rust_analyzer" , None) ,] ;
};
}

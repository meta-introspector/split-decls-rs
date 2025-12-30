// Generated macro for TargetCfg (struct)
macro_rules! Depcrate_commonTargetCfg {
() => {
// Module: crate::common
// Provides: {"TargetCfg"}
// Dependencies: {}
# [derive (Clone , Debug , serde :: Deserialize)] # [serde (rename_all = "kebab-case")] pub struct TargetCfg { pub (crate) arch : String , # [serde (default = "default_os")] pub (crate) os : String , # [serde (default)] pub (crate) env : String , # [serde (default)] pub (crate) abi : String , # [serde (rename = "target-family" , default)] pub (crate) families : Vec < String > , # [serde (rename = "target-pointer-width")] pub (crate) pointer_width : u32 , # [serde (rename = "target-endian" , default)] endian : Endian , # [serde (rename = "panic-strategy" , default)] pub (crate) panic : PanicStrategy , # [serde (default)] pub (crate) dynamic_linking : bool , # [serde (rename = "supported-sanitizers" , default)] pub (crate) sanitizers : Vec < Sanitizer > , # [serde (rename = "supports-xray" , default)] pub (crate) xray : bool , # [serde (default = "default_reloc_model")] pub (crate) relocation_model : String , pub (crate) rustc_abi : Option < String > , # [serde (skip)] # [doc = " Supported target atomic widths: e.g. `8` to `128` or `ptr`. This is derived from the builtin"] # [doc = " `target_has_atomic` `cfg`s e.g. `target_has_atomic=\"8\"`."] pub (crate) target_has_atomic : BTreeSet < String > , }
};
}

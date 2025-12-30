// Generated macro for ExtFuncData (struct)
macro_rules! Depcrate_ir_extfuncExtFuncData {
() => {
// Module: crate::ir::extfunc
// Provides: {"ExtFuncData"}
// Dependencies: {}
# [doc = " An external function."] # [doc = ""] # [doc = " Information about a function that can be called directly with a direct `call` instruction."] # [derive (Clone , Debug , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct ExtFuncData { # [doc = " Name of the external function."] pub name : ExternalName , # [doc = " Call signature of function."] pub signature : SigRef , # [doc = " Will this function be defined nearby, such that it will always be a certain distance away,"] # [doc = " after linking? If so, references to it can avoid going through a GOT or PLT. Note that"] # [doc = " symbols meant to be preemptible cannot be considered colocated."] # [doc = ""] # [doc = " If `true`, some backends may use relocation forms that have limited range. The exact"] # [doc = " distance depends on the code model in use. Currently on AArch64, for example, Cranelift"] # [doc = " uses a custom code model supporting up to +/- 128MB displacements. If it is unknown how"] # [doc = " far away the target will be, it is best not to set the `colocated` flag; in general, this"] # [doc = " flag is best used when the target is known to be in the same unit of code generation, such"] # [doc = " as a Wasm module."] # [doc = ""] # [doc = " See the documentation for `RelocDistance` for more details. A `colocated` flag value of"] # [doc = " `true` implies `RelocDistance::Near`."] pub colocated : bool , }
};
}

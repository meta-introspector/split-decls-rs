// Generated macro for Compiler (struct)
macro_rules! DepcrateCompiler {
() => {
// Module: crate
// Provides: {"Compiler"}
// Dependencies: {}
# [doc = " A structure representing a Rust compiler."] # [doc = ""] # [doc = " Each compiler has a `stage` that it is associated with and a `host` that"] # [doc = " corresponds to the platform the compiler runs on. This structure is used as"] # [doc = " a parameter to many methods below."] # [derive (Eq , PartialOrd , Ord , Clone , Copy , Debug)] pub struct Compiler { stage : u32 , host : TargetSelection , # [doc = " Indicates whether the compiler was forced to use a specific stage."] # [doc = " This field is ignored in `Hash` and `PartialEq` implementations as only the `stage`"] # [doc = " and `host` fields are relevant for those."] forced_compiler : bool , }
};
}

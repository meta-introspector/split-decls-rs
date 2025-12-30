// Generated macro for impl_526 (impl)
macro_rules! Depcrate_core_build_steps_setupimpl_526 {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"impl_526"}
// Dependencies: {}
impl FromStr for Profile { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "lib" | "library" => Ok (Profile :: Library) , "compiler" => Ok (Profile :: Compiler) , "maintainer" | "dist" | "user" => Ok (Profile :: Dist) , "tools" | "tool" | "rustdoc" | "clippy" | "miri" | "rustfmt" => Ok (Profile :: Tools) , "none" => Ok (Profile :: None) , "llvm" | "codegen" => Err ("the \"llvm\" and \"codegen\" profiles have been removed,\
                use \"compiler\" instead which has the same functionality" . to_string ()) , _ => Err (format ! ("unknown profile: '{s}'")) , } } }
};
}

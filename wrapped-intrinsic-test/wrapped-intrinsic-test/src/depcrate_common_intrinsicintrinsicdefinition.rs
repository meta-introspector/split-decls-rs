// Generated macro for IntrinsicDefinition (trait)
macro_rules! Depcrate_common_intrinsicIntrinsicDefinition {
() => {
// Module: crate::common::intrinsic
// Provides: {"IntrinsicDefinition"}
// Dependencies: {}
pub trait IntrinsicDefinition < T > where T : IntrinsicTypeDefinition , { fn arguments (& self) -> ArgumentList < T > ; fn results (& self) -> T ; fn name (& self) -> String ; # [doc = " Generates a std::cout for the intrinsics results that will match the"] # [doc = " rust debug output format for the return type. The generated line assumes"] # [doc = " there is an int i in scope which is the current pass number."] fn print_result_c (& self , _indentation : Indentation , _additional : & str) -> String ; }
};
}

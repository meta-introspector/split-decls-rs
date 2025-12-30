// Generated macro for JsonIntrinsic (struct)
macro_rules! Depcrate_arm_json_parserJsonIntrinsic {
() => {
// Module: crate::arm::json_parser
// Provides: {"JsonIntrinsic"}
// Dependencies: {}
# [derive (Deserialize , Debug)] struct JsonIntrinsic { # [serde (rename = "SIMD_ISA")] simd_isa : String , name : String , arguments : Vec < String > , return_type : ReturnType , # [serde (rename = "Arguments_Preparation")] args_prep : Option < HashMap < String , Value > > , # [serde (rename = "Architectures")] architectures : Vec < String > , }
};
}

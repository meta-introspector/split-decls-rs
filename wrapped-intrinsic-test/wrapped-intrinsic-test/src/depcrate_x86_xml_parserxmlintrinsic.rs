// Generated macro for XMLIntrinsic (struct)
macro_rules! Depcrate_x86_xml_parserXMLIntrinsic {
() => {
// Module: crate::x86::xml_parser
// Provides: {"XMLIntrinsic"}
// Dependencies: {}
# [derive (Deserialize)] struct XMLIntrinsic { # [serde (rename = "return")] pub return_data : Parameter , # [serde (rename = "@name")] pub name : String , # [serde (rename = "@tech")] tech : String , # [serde (rename = "CPUID" , default)] cpuid : Vec < String > , # [serde (rename = "parameter" , default)] parameters : Vec < Parameter > , }
};
}

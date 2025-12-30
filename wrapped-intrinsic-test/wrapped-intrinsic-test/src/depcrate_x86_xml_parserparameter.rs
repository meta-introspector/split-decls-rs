// Generated macro for Parameter (struct)
macro_rules! Depcrate_x86_xml_parserParameter {
() => {
// Module: crate::x86::xml_parser
// Provides: {"Parameter"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Deserialize)] pub struct Parameter { # [serde (rename = "@varname" , default)] pub var_name : String , # [serde (rename = "@type" , default)] pub type_data : String , # [serde (rename = "@etype" , default)] pub etype : String , # [serde (rename = "@memwidth" , default , deserialize_with = "string_to_u32")] pub memwidth : u32 , # [serde (rename = "@immwidth" , default , deserialize_with = "string_to_u32")] pub imm_width : u32 , # [serde (rename = "@immtype" , default)] pub imm_type : String , }
};
}

// Generated macro for as_hex (function)
macro_rules! Depcrate_content_yaml_vendored_scanneras_hex {
() => {
// Module: crate::content::yaml::vendored::scanner
// Provides: {"as_hex"}
// Dependencies: {}
# [inline] fn as_hex (c : char) -> u32 { match c { '0' ..= '9' => (c as u32) - ('0' as u32) , 'a' ..= 'f' => (c as u32) - ('a' as u32) + 10 , 'A' ..= 'F' => (c as u32) - ('A' as u32) + 10 , _ => unreachable ! () , } }
};
}

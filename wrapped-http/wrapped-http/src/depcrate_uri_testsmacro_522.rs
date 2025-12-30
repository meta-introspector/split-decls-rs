// Generated macro for macro_522 (macro)
macro_rules! Depcrate_uri_testsmacro_522 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_522"}
// Dependencies: {}
test_parse ! { test_uri_parse_long_host_with_port_and_no_scheme , "thequickbrownfoxjumpedoverthelazydogtofindthelargedangerousdragon.localhost:1234" , [] , scheme = None , authority = part ! ("thequickbrownfoxjumpedoverthelazydogtofindthelargedangerousdragon.localhost:1234") , path = "" , query = None , port = Port :: from_str ("1234") . ok () , }
};
}

// Generated macro for ClientArgs (struct)
macro_rules! Depcrate_argsClientArgs {
() => {
// Module: crate::args
// Provides: {"ClientArgs"}
// Dependencies: {}
# [doc = " Application-specific arguments that compliment the `CommonArgs`."] pub struct ClientArgs { pub version : u32 , pub dump_response_path : Option < String > , pub dump_json : Option < usize > , pub urls : Vec < url :: Url > , pub reqs_cardinal : u64 , pub req_headers : Vec < String > , pub no_verify : bool , pub trust_origin_ca_pem : Option < String > , pub body : Option < Vec < u8 > > , pub method : String , pub connect_to : Option < String > , pub session_file : Option < String > , pub source_port : u16 , pub perform_migration : bool , pub send_priority_update : bool , }
};
}

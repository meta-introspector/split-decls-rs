// Generated macro for Mutation (struct)
macro_rules! Depcrate_registryMutation {
() => {
// Module: crate::registry
// Provides: {"Mutation"}
// Dependencies: {}
# [doc = " A helper struct that collects the arguments for [`HttpServer::check_authorized`]."] # [doc = " Based on looking at the request, these are the fields that the authentication header should attest to."] struct Mutation < 'a > { mutation : & 'a str , name : Option < & 'a str > , vers : Option < & 'a str > , cksum : Option < & 'a str > , }
};
}

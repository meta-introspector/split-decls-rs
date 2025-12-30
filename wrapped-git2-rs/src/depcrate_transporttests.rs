// Generated macro for tests (module)
macro_rules! Depcrate_transporttests {
() => {
// Module: crate::transport
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: { ErrorClass , ErrorCode } ; use std :: sync :: Once ; struct DummyTransport ; fn dummy_error () -> Error { Error :: new (ErrorCode :: Ambiguous , ErrorClass :: Net , "bleh") } impl SmartSubtransport for DummyTransport { fn action (& self , _url : & str , _service : Service ,) -> Result < Box < dyn SmartSubtransportStream > , Error > { Err (dummy_error ()) } fn close (& self) -> Result < () , Error > { Ok (()) } } # [test] fn transport_error_propagates () { static INIT : Once = Once :: new () ; unsafe { INIT . call_once (| | { register ("dummy" , move | remote | { Transport :: smart (& remote , true , DummyTransport) }) . unwrap () ; }) } let (_td , repo) = crate :: test :: repo_init () ; t ! (repo . remote ("origin" , "dummy://ball")) ; let mut origin = t ! (repo . find_remote ("origin")) ; match origin . fetch (& ["main"] , None , None) { Ok (()) => unreachable ! () , Err (e) => assert_eq ! (e , dummy_error ()) , } } }
};
}

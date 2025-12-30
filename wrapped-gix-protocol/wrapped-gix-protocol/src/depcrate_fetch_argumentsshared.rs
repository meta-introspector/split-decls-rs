// Generated macro for shared (module)
macro_rules! Depcrate_fetch_argumentsshared {
() => {
// Module: crate::fetch::arguments
// Provides: {"shared"}
// Dependencies: {}
# [cfg (any (feature = "blocking-client" , feature = "async-client"))] mod shared { use bstr :: { BString , ByteSlice } ; use gix_transport :: { client , client :: MessageKind } ; use crate :: fetch :: Arguments ; impl Arguments { pub (in crate :: fetch :: arguments) fn prepare_v1 (& mut self , transport_is_stateful : bool , add_done_argument : bool ,) -> Result < (MessageKind , Option < Vec < BString > >) , client :: Error > { if self . haves . is_empty () { assert ! (add_done_argument , "If there are no haves, is_done must be true.") ; } let on_into_read = if add_done_argument { client :: MessageKind :: Text (& b"done" [..]) } else { client :: MessageKind :: Flush } ; let retained_state = if transport_is_stateful { None } else { Some (self . args . clone ()) } ; if let Some (first_arg_position) = self . args . iter () . position (| l | l . starts_with_str ("want ")) { self . args . swap (first_arg_position , 0) ; } Ok ((on_into_read , retained_state)) } } }
};
}

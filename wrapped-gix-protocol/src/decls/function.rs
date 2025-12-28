macro_rules! deps {
    () => {
        Action!();
        Error!();
        Command!();
        Ref!();
    };
}

macro_rules! function {
    () => {
        deps!();
        # [cfg (any (feature = "blocking-client" , feature = "async-client"))] pub (crate) mod function { use std :: borrow :: Cow ; use bstr :: BString ; use gix_features :: progress :: Progress ; use gix_transport :: client :: Capabilities ; use maybe_async :: maybe_async ; use super :: { Action , Error } ; # [cfg (feature = "async-client")] use crate :: transport :: client :: async_io :: { Transport , TransportV2Ext } ; # [cfg (feature = "blocking-client")] use crate :: transport :: client :: blocking_io :: { Transport , TransportV2Ext } ; use crate :: { handshake :: { refs :: from_v2_refs , Ref } , indicate_end_of_interaction , Command , } ; # [doc = " Invoke a ls-refs V2 command on `transport`, which requires a prior handshake that yielded"] # [doc = " server `capabilities`. `prepare_ls_refs(capabilities, arguments)` can be used to alter the _ls-refs_."] # [doc = " `progress` is used to provide feedback."] # [doc = " The `agent` information will be added to the features sent to the server."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] # [maybe_async] pub async fn ls_refs (mut transport : impl Transport , capabilities : & Capabilities , prepare_ls_refs : impl FnOnce (& Capabilities , & mut Vec < BString >) -> std :: io :: Result < Action > , progress : & mut impl Progress , trace : bool , agent : (& 'static str , Option < Cow < 'static , str > >) ,) -> Result < Vec < Ref > , Error > { let _span = gix_features :: trace :: detail ! ("gix_protocol::ls_refs()" , capabilities = ? capabilities) ; let ls_refs = Command :: LsRefs ; let mut ls_features = ls_refs . default_features (gix_transport :: Protocol :: V2 , capabilities) ; ls_features . push (agent) ; let mut ls_args = ls_refs . initial_v2_arguments (& ls_features) ; if capabilities . capability ("ls-refs") . and_then (| cap | cap . supports ("unborn")) . unwrap_or_default () { ls_args . push ("unborn" . into ()) ; } let refs = match prepare_ls_refs (capabilities , & mut ls_args) { Ok (Action :: Skip) => Vec :: new () , Ok (Action :: Continue) => { ls_refs . validate_argument_prefixes (gix_transport :: Protocol :: V2 , capabilities , & ls_args , & ls_features ,) ? ; progress . step () ; progress . set_name ("list refs" . into ()) ; let mut remote_refs = transport . invoke (ls_refs . as_str () , ls_features . into_iter () , if ls_args . is_empty () { None } else { Some (ls_args . into_iter ()) } , trace ,) . await ? ; from_v2_refs (& mut remote_refs) . await ? } Err (err) => { indicate_end_of_interaction (transport , trace) . await ? ; return Err (err . into ()) ; } } ; Ok (refs) } }
    };
}

function!()
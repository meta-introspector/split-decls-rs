macro_rules! deps {
    () => {
        Shallow!();
        Context!();
        Ref!();
        Error!();
    };
}

macro_rules! hero {
    () => {
        deps!();
        # [cfg (feature = "handshake")] pub (crate) mod hero { use crate :: handshake :: Ref ; # [doc = " The result of the [`handshake()`](crate::handshake()) function."] # [derive (Default , Debug , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Handshake { # [doc = " The protocol version the server responded with. It might have downgraded the desired version."] pub server_protocol_version : gix_transport :: Protocol , # [doc = " The references reported as part of the `Protocol::V1` handshake, or `None` otherwise as V2 requires a separate request."] pub refs : Option < Vec < Ref > > , # [doc = " Shallow updates as part of the `Protocol::V1`, to shallow a particular object."] # [doc = " Note that unshallowing isn't supported here."] pub v1_shallow_updates : Option < Vec < crate :: fetch :: response :: ShallowUpdate > > , # [doc = " The server capabilities."] pub capabilities : gix_transport :: client :: Capabilities , } # [cfg (feature = "fetch")] mod fetch { # [cfg (feature = "async-client")] use crate :: transport :: client :: async_io :: Transport ; # [cfg (feature = "blocking-client")] use crate :: transport :: client :: blocking_io :: Transport ; use crate :: Handshake ; use gix_features :: progress :: Progress ; use std :: borrow :: Cow ; impl Handshake { # [doc = " Obtain a [refmap](crate::fetch::RefMap) either from this instance, taking it out in the process, if the handshake was"] # [doc = " created from a V1 connection, or use `transport` to fetch the refmap as a separate command invocation."] # [allow (clippy :: result_large_err)] # [maybe_async :: maybe_async] pub async fn fetch_or_extract_refmap < T > (& mut self , mut progress : impl Progress , transport : & mut T , user_agent : (& 'static str , Option < Cow < 'static , str > >) , trace_packetlines : bool , prefix_from_spec_as_filter_on_remote : bool , refmap_context : crate :: fetch :: refmap :: init :: Context ,) -> Result < crate :: fetch :: RefMap , crate :: fetch :: refmap :: init :: Error > where T : Transport , { Ok (match self . refs . take () { Some (refs) => crate :: fetch :: RefMap :: from_refs (refs , & self . capabilities , refmap_context) ? , None => { crate :: fetch :: RefMap :: fetch (& mut progress , & self . capabilities , transport , user_agent , trace_packetlines , prefix_from_spec_as_filter_on_remote , refmap_context ,) . await ? } }) } } } }
    };
}

hero!();
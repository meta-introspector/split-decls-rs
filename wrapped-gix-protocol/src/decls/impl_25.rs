macro_rules! deps {
    () => {
        Response!();
        Feature!();
        Error!();
        Acknowledgement!();
        WantedRef!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Response { # [doc = " Return true if the response has a pack which can be read next."] pub fn has_pack (& self) -> bool { self . has_pack } # [doc = " Return an error if the given `features` don't contain the required ones (the ones this implementation needs)"] # [doc = " for the given `version` of the protocol."] # [doc = ""] # [doc = " Even though technically any set of features supported by the server could work, we only implement the ones that"] # [doc = " make it easy to maintain all versions with a single code base that aims to be and remain maintainable."] pub fn check_required_features (version : Protocol , features : & [Feature]) -> Result < () , Error > { match version { Protocol :: V0 | Protocol :: V1 => { let has = | name : & str | features . iter () . any (| f | f . 0 == name) ; if ! has ("multi_ack_detailed") { return Err (Error :: MissingServerCapability { feature : "multi_ack_detailed" , }) ; } if ! has ("side-band") && ! has ("side-band-64k") { return Err (Error :: MissingServerCapability { feature : "side-band OR side-band-64k" , }) ; } } Protocol :: V2 => { } } Ok (()) } # [doc = " Return all acknowledgements [parsed previously][Response::from_line_reader()]."] pub fn acknowledgements (& self) -> & [Acknowledgement] { & self . acks } # [doc = " Return all shallow update lines [parsed previously][Response::from_line_reader()]."] pub fn shallow_updates (& self) -> & [ShallowUpdate] { & self . shallows } # [doc = " Append the given `updates` which may have been obtained from a"] # [doc = " (handshake::Outcome)[crate::Handshake::v1_shallow_updates]."] # [doc = ""] # [doc = " In V2, these are received as part of the pack, but V1 sends them early, so we"] # [doc = " offer to re-integrate them here."] pub fn append_v1_shallow_updates (& mut self , updates : Option < Vec < ShallowUpdate > >) { self . shallows . extend (updates . into_iter () . flatten ()) ; } # [doc = " Return all wanted-refs [parsed previously][Response::from_line_reader()]."] pub fn wanted_refs (& self) -> & [WantedRef] { & self . wanted_refs } }
    };
}

impl_25!()
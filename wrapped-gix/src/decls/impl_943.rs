macro_rules! deps {
    () => {
        Name!();
        Connection!();
        Fetch!();
        Error!();
        Options!();
        Note!();
        Direction!();
    };
}

macro_rules! impl_943 {
    () => {
        deps!();
        impl < T > Connection < '_ , '_ , T > where T : Transport , { # [doc = " List all references on the remote that have been filtered through our remote's [`refspecs`][crate::Remote::refspecs()]"] # [doc = " for _fetching_."] # [doc = ""] # [doc = " This comes in the form of all matching tips on the remote and the object they point to, along with"] # [doc = " the local tracking branch of these tips (if available)."] # [doc = ""] # [doc = " Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository."] # [doc = ""] # [doc = " # Consumption"] # [doc = ""] # [doc = " Due to management of the transport, it's cleanest to only use it for a single interaction. Thus, it's consumed"] # [doc = " along with the connection."] # [doc = ""] # [doc = " ### Configuration"] # [doc = ""] # [doc = " - `gitoxide.userAgent` is read to obtain the application user agent for git servers and for HTTP servers as well."] # [allow (clippy :: result_large_err)] # [gix_protocol :: maybe_async :: maybe_async] pub async fn ref_map (mut self , progress : impl Progress , options : Options ,) -> Result < (fetch :: RefMap , gix_protocol :: Handshake) , Error > { let refmap = self . ref_map_by_ref (progress , options) . await ? ; let handshake = self . handshake . expect ("refmap always performs handshake and stores it if it succeeds") ; Ok ((refmap , handshake)) } # [allow (clippy :: result_large_err)] # [gix_protocol :: maybe_async :: maybe_async] pub (crate) async fn ref_map_by_ref (& mut self , mut progress : impl Progress , Options { prefix_from_spec_as_filter_on_remote , handshake_parameters , mut extra_refspecs , } : Options ,) -> Result < fetch :: RefMap , Error > { let _span = gix_trace :: coarse ! ("remote::Connection::ref_map()") ; if let Some (tag_spec) = self . remote . fetch_tags . to_refspec () . map (| spec | spec . to_owned ()) { if ! extra_refspecs . contains (& tag_spec) { extra_refspecs . push (tag_spec) ; } } let mut credentials_storage ; let url = self . transport . inner . to_url () ; let authenticate = match self . authenticate . as_mut () { Some (f) => f , None => { let url = self . remote . url (Direction :: Fetch) . map_or_else (| | gix_url :: parse (url . as_ref ()) . expect ("valid URL to be provided by transport") , ToOwned :: to_owned ,) ; credentials_storage = self . configured_credentials (url) ? ; & mut credentials_storage } } ; let repo = self . remote . repo ; if self . transport_options . is_none () { self . transport_options = repo . transport_options (url . as_ref () , self . remote . name () . map (crate :: remote :: Name :: as_bstr)) . map_err (| err | Error :: GatherTransportConfig { source : err , url : url . into_owned () , }) ? ; } if let Some (config) = self . transport_options . as_ref () { self . transport . inner . configure (& * * config) ? ; } let mut handshake = gix_protocol :: handshake (& mut self . transport . inner , gix_transport :: Service :: UploadPack , authenticate , handshake_parameters , & mut progress ,) . await ? ; let context = fetch :: refmap :: init :: Context { fetch_refspecs : self . remote . fetch_specs . clone () , extra_refspecs , } ; let ref_map = handshake . fetch_or_extract_refmap (progress , & mut self . transport . inner , self . remote . repo . config . user_agent_tuple () , self . trace , prefix_from_spec_as_filter_on_remote , context ,) . await ? ; self . handshake = Some (handshake) ; Ok (ref_map) } }
    };
}

impl_943!();
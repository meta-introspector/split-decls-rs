macro_rules! deps {
    () => {
        Remote!();
        Error!();
        Note!();
        Url!();
        Options!();
        Direction!();
        Path!();
        Ssh!();
        Protocol!();
        Connection!();
    };
}

macro_rules! impl_931 {
    () => {
        deps!();
        # [doc = " Establishing connections to remote hosts (without performing a git-handshake)."] impl < 'repo > Remote < 'repo > { # [doc = " Create a new connection using `transport` to communicate, with `progress` to indicate changes."] # [doc = ""] # [doc = " Note that this method expects the `transport` to be created by the user, which would involve the [`url()`](Self::url())."] # [doc = " It's meant to be used when async operation is needed with runtimes of the user's choice."] pub fn to_connection_with_transport < T > (& self , transport : T) -> Connection < '_ , 'repo , T > where T : Transport , { let trace = self . repo . config . trace_packet () ; Connection { remote : self , authenticate : None , transport_options : None , handshake : None , transport : gix_protocol :: SendFlushOnDrop :: new (transport , trace) , trace , } } # [doc = " Connect to the url suitable for `direction` and return a handle through which operations can be performed."] # [doc = ""] # [doc = " Note that the `protocol.version` configuration key affects the transport protocol used to connect,"] # [doc = " with `2` being the default."] # [doc = ""] # [doc = " The transport used for connection can be configured via `transport_mut().configure()` assuming the actually"] # [doc = " used transport is well known. If that's not the case, the transport can be created by hand and passed to"] # [doc = " [to_connection_with_transport()][Self::to_connection_with_transport()]."] # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client-async-std"))] # [gix_protocol :: maybe_async :: maybe_async] pub async fn connect (& self , direction : crate :: remote :: Direction ,) -> Result < Connection < '_ , 'repo , Box < dyn Transport + Send > > , Error > { let (url , version) = self . sanitized_url_and_version (direction) ? ; # [cfg (feature = "blocking-network-client")] let scheme_is_ssh = url . scheme == gix_url :: Scheme :: Ssh ; let transport = connect :: connect (url , connect :: Options { version , # [cfg (feature = "blocking-network-client")] ssh : scheme_is_ssh . then (| | self . repo . ssh_connect_options ()) . transpose () ? . unwrap_or_default () , trace : self . repo . config . trace_packet () , } ,) . await ? ; Ok (self . to_connection_with_transport (transport)) } # [doc = " Produce the sanitized URL and protocol version to use as obtained by querying the repository configuration."] # [doc = ""] # [doc = " This can be useful when using custom transports to allow additional configuration."] pub fn sanitized_url_and_version (& self , direction : crate :: remote :: Direction ,) -> Result < (gix_url :: Url , gix_protocol :: transport :: Protocol) , Error > { fn sanitize (mut url : gix_url :: Url) -> Result < gix_url :: Url , Error > { if url . scheme == gix_url :: Scheme :: File { let mut dir = gix_path :: to_native_path_on_windows (Cow :: Borrowed (url . path . as_ref ())) ; let kind = gix_discover :: is_git (dir . as_ref ()) . or_else (| _ | { dir . to_mut () . push (gix_discover :: DOT_GIT_DIR) ; gix_discover :: is_git (dir . as_ref ()) }) . map_err (| err | Error :: FileUrl { source : err . into () , url : url . clone () , }) ? ; let (git_dir , _work_dir) = gix_discover :: repository :: Path :: from_dot_git_dir (dir . clone () . into_owned () , kind , & gix_fs :: current_dir (false) ? ,) . ok_or_else (| | Error :: InvalidRemoteRepositoryPath { directory : dir . into_owned () , }) ? . into_repository_and_work_tree_directories () ; url . path = gix_path :: into_bstr (git_dir) . into_owned () ; } Ok (url) } let version = crate :: config :: tree :: Protocol :: VERSION . try_into_protocol_version (self . repo . config . resolved . integer (Protocol :: VERSION)) . map_err (| err | Error :: UnknownProtocol { source : err }) ? ; let url = self . url (direction) . ok_or (Error :: MissingUrl { direction }) ? . to_owned () ; if ! self . repo . config . url_scheme () ? . allow (& url . scheme) { return Err (Error :: ProtocolDenied { url : url . to_bstring () , scheme : url . scheme , }) ; } Ok ((sanitize (url) ? , version)) } }
    };
}

impl_931!();
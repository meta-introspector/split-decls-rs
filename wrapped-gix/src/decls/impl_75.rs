macro_rules! deps {
    () => {
        Kind!();
        Path!();
        Error!();
        Options!();
        ThreadSafeRepository!();
        PrepareFetch!();
        Url!();
        Note!();
        Default!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        # [doc = " Instantiation"] impl PrepareFetch { # [doc = " Create a new repository at `path` with `create_opts` which is ready to clone from `url`, possibly after making additional adjustments to"] # [doc = " configuration and settings."] # [doc = ""] # [doc = " Note that this is merely a handle to perform the actual connection to the remote, and if any of it fails the freshly initialized repository"] # [doc = " will be removed automatically as soon as this instance drops."] # [doc = ""] # [doc = " # Deviation"] # [doc = ""] # [doc = " Similar to `git`, a missing user name and email configuration is not terminal and we will fill it in with dummy values. However,"] # [doc = " instead of deriving values from the system, ours are hardcoded to indicate what happened."] # [allow (clippy :: result_large_err)] pub fn new < Url , E > (url : Url , path : impl AsRef < std :: path :: Path > , kind : crate :: create :: Kind , create_opts : crate :: create :: Options , open_opts : crate :: open :: Options ,) -> Result < Self , Error > where Url : TryInto < gix_url :: Url , Error = E > , gix_url :: parse :: Error : From < E > , { Self :: new_inner (url . try_into () . map_err (gix_url :: parse :: Error :: from) ? , path . as_ref () , kind , create_opts , open_opts ,) } # [allow (clippy :: result_large_err)] fn new_inner (mut url : gix_url :: Url , path : & std :: path :: Path , kind : crate :: create :: Kind , mut create_opts : crate :: create :: Options , open_opts : crate :: open :: Options ,) -> Result < Self , Error > { create_opts . destination_must_be_empty = true ; let mut repo = crate :: ThreadSafeRepository :: init_opts (path , kind , create_opts , open_opts) ? . to_thread_local () ; url . canonicalize (repo . options . current_dir_or_empty ()) . map_err (| err | Error :: CanonicalizeUrl { url : url . clone () , source : err , }) ? ; repo . committer_or_set_generic_fallback () ? ; Ok (PrepareFetch { url , # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] fetch_options : Default :: default () , repo : Some (repo) , config_overrides : Vec :: new () , remote_name : None , configure_remote : None , # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] configure_connection : None , shallow : remote :: fetch :: Shallow :: NoChange , ref_name : None , }) } }
    };
}

impl_75!();
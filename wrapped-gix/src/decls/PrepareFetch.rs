macro_rules! deps {
    () => {
        Url!();
        ConfigureConnectionFn!();
        Repository!();
        Options!();
        ConfigureRemoteFn!();
    };
}

macro_rules! PrepareFetch {
    () => {
        deps!();
        # [doc = " A utility to collect configuration on how to fetch from a remote and initiate a fetch operation. It will delete the newly"] # [doc = " created repository on when dropped without successfully finishing a fetch."] # [must_use] pub struct PrepareFetch { # [doc = " A freshly initialized repository which is owned by us, or `None` if it was handed to the user"] repo : Option < crate :: Repository > , # [doc = " The name of the remote, which defaults to `origin` if not overridden."] remote_name : Option < BString > , # [doc = " Additional config `values` that are applied in-memory before starting the fetch process."] config_overrides : Vec < BString > , # [doc = " A function to configure a remote prior to fetching a pack."] configure_remote : Option < ConfigureRemoteFn > , # [doc = " A function to configure a connection before using it."] # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] configure_connection : Option < ConfigureConnectionFn > , # [doc = " Options for preparing a fetch operation."] # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] fetch_options : remote :: ref_map :: Options , # [doc = " The url to clone from"] # [cfg_attr (not (feature = "blocking-network-client") , allow (dead_code))] url : gix_url :: Url , # [doc = " How to handle shallow clones"] # [cfg_attr (not (feature = "blocking-network-client") , allow (dead_code))] shallow : remote :: fetch :: Shallow , # [doc = " The name of the reference to fetch. If `None`, the reference pointed to by `HEAD` will be checked out."] # [cfg_attr (not (feature = "blocking-network-client") , allow (dead_code))] ref_name : Option < gix_ref :: PartialName > , }
    };
}

PrepareFetch!();
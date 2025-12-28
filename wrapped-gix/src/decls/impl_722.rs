macro_rules! deps {
    () => {
        ProxyAuthMethod!();
        SubSectionRequirement!();
        FetchRefSpec!();
        PushRefSpec!();
        Remote!();
        Tree!();
        TagOpt!();
        Url!();
        RemoteName!();
        String!();
    };
}

macro_rules! impl_722 {
    () => {
        deps!();
        impl Remote { # [doc = " The `remote.pushDefault` key"] pub const PUSH_DEFAULT : keys :: RemoteName = keys :: RemoteName :: new_remote_name ("pushDefault" , & config :: Tree :: REMOTE) ; # [doc = " The `remote.<name>.tagOpt` key"] pub const TAG_OPT : TagOpt = TagOpt :: new_with_validate ("tagOpt" , & config :: Tree :: REMOTE , validate :: TagOpt) . with_subsection_requirement (Some (SubSectionRequirement :: Parameter ("name"))) ; # [doc = " The `remote.<name>.url` key"] pub const URL : keys :: Url = keys :: Url :: new_url ("url" , & config :: Tree :: REMOTE) . with_subsection_requirement (NAME_PARAMETER) ; # [doc = " The `remote.<name>.pushUrl` key"] pub const PUSH_URL : keys :: Url = keys :: Url :: new_url ("pushUrl" , & config :: Tree :: REMOTE) . with_subsection_requirement (NAME_PARAMETER) ; # [doc = " The `remote.<name>.fetch` key"] pub const FETCH : keys :: FetchRefSpec = keys :: FetchRefSpec :: new_fetch_refspec ("fetch" , & config :: Tree :: REMOTE) . with_subsection_requirement (NAME_PARAMETER) ; # [doc = " The `remote.<name>.push` key"] pub const PUSH : keys :: PushRefSpec = keys :: PushRefSpec :: new_push_refspec ("push" , & config :: Tree :: REMOTE) . with_subsection_requirement (NAME_PARAMETER) ; # [doc = " The `remote.<name>.proxy` key"] pub const PROXY : keys :: String = keys :: String :: new_string ("proxy" , & config :: Tree :: REMOTE) . with_subsection_requirement (NAME_PARAMETER) ; # [doc = " The `remote.<name>.proxyAuthMethod` key."] pub const PROXY_AUTH_METHOD : http :: ProxyAuthMethod = http :: ProxyAuthMethod :: new_proxy_auth_method ("proxyAuthMethod" , & config :: Tree :: REMOTE) . with_subsection_requirement (NAME_PARAMETER) . with_deviation ("implemented like git, but never actually tried") ; }
    };
}

impl_722!();
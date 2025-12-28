macro_rules! deps {
    () => {
        RemoteRedirect!();
    };
}

macro_rules! impl_662 {
    () => {
        deps!();
        impl RemoteRedirect { fn raw (& self) -> raw :: git_remote_redirect_t { match self { RemoteRedirect :: None => raw :: GIT_REMOTE_REDIRECT_NONE , RemoteRedirect :: Initial => raw :: GIT_REMOTE_REDIRECT_INITIAL , RemoteRedirect :: All => raw :: GIT_REMOTE_REDIRECT_ALL , } } }
    };
}

impl_662!()
macro_rules! deps {
    () => {
        Note!();
    };
}

macro_rules! agent {
    () => {
        deps!();
        # [doc = " Returns the name of the agent for identification towards a remote server as statically known when compiling the crate."] # [doc = " Suitable for both `git` servers and HTTP servers, and used unless configured otherwise."] # [doc = ""] # [doc = " Note that it's meant to be used in conjunction with [`protocol::agent()`][crate::protocol::agent()] which"] # [doc = " prepends `git/`."] pub fn agent () -> & 'static str { concat ! ("oxide-" , env ! ("CARGO_PKG_VERSION")) }
    };
}

agent!();
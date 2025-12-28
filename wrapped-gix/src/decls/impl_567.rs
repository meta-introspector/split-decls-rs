macro_rules! deps {
    () => {
        Snapshot!();
    };
}

macro_rules! impl_567 {
    () => {
        deps!();
        # [doc = " Utilities and additional access"] impl Snapshot < '_ > { # [doc = " Returns the underlying configuration implementation for a complete API, despite being a little less convenient."] # [doc = ""] # [doc = " It's expected that more functionality will move up depending on demand."] pub fn plumbing (& self) -> & gix_config :: File < 'static > { & self . repo . config . resolved } }
    };
}

impl_567!();
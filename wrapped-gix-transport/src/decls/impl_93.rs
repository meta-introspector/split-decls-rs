macro_rules! deps {
    () => {
        Http!();
        Protocol!();
        Transport!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [cfg (any (feature = "http-client-curl" , feature = "http-client-reqwest"))] impl < H : Http + Default > Transport < H > { # [doc = " Create a new instance to communicate to `url` using the given `desired_version` of the `git` protocol."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] # [doc = ""] # [doc = " Note that the actual implementation depends on feature toggles."] pub fn new (url : gix_url :: Url , desired_version : Protocol , trace : bool) -> Self { Self :: new_http (H :: default () , url , desired_version , trace) } }
    };
}

impl_93!();
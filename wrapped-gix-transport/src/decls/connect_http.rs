macro_rules! deps {
    () => {
        Http!();
        Transport!();
        Protocol!();
    };
}

macro_rules! connect_http {
    () => {
        deps!();
        # [doc = " Connect to the given `url` via HTTP/S using the `desired_version` of the `git` protocol, with `http` as implementation."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] # [cfg (all (feature = "http-client" , not (feature = "http-client-curl")))] pub fn connect_http < H : Http > (http : H , url : gix_url :: Url , desired_version : Protocol , trace : bool) -> Transport < H > { Transport :: new_http (http , url , desired_version , trace) }
    };
}

connect_http!();
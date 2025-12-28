macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! encode_engine {
    () => {
        deps!();
        # [doc = "Encode arbitrary octets as base64 using the provided `Engine` into a new `String`."] # [doc = ""] # [doc = " See [`Engine::encode`]."] # [allow (unused)] # [deprecated (since = "0.21.0" , note = "Use Engine::encode")] # [cfg (any (feature = "alloc" , test))] pub fn encode_engine < E : Engine , T : AsRef < [u8] > > (input : T , engine : & E) -> String { engine . encode (input) }
    };
}

encode_engine!()
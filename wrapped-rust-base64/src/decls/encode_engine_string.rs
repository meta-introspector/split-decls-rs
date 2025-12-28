macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! encode_engine_string {
    () => {
        deps!();
        # [doc = "Encode arbitrary octets as base64 into a supplied `String`."] # [doc = ""] # [doc = " See [`Engine::encode_string`]."] # [allow (unused)] # [deprecated (since = "0.21.0" , note = "Use Engine::encode_string")] # [cfg (any (feature = "alloc" , test))] pub fn encode_engine_string < E : Engine , T : AsRef < [u8] > > (input : T , output_buf : & mut String , engine : & E ,) { engine . encode_string (input , output_buf) ; }
    };
}

encode_engine_string!()
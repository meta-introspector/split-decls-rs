macro_rules! deps {
    () => {
        Engine!();
        DecodeError!();
    };
}

macro_rules! decode_engine {
    () => {
        deps!();
        # [doc = " Decode from string reference as octets using the specified [Engine]."] # [doc = ""] # [doc = " See [`Engine::decode`]."] # [doc = "Returns a `Result` containing a `Vec<u8>`."] # [deprecated (since = "0.21.0" , note = "Use Engine::decode")] # [cfg (any (feature = "alloc" , test))] pub fn decode_engine < E : Engine , T : AsRef < [u8] > > (input : T , engine : & E ,) -> Result < Vec < u8 > , DecodeError > { engine . decode (input) }
    };
}

decode_engine!();
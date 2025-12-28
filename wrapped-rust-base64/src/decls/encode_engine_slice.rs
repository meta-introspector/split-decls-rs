macro_rules! deps {
    () => {
        EncodeSliceError!();
        Engine!();
    };
}

macro_rules! encode_engine_slice {
    () => {
        deps!();
        # [doc = " Encode arbitrary octets as base64 into a supplied slice."] # [doc = ""] # [doc = " See [`Engine::encode_slice`]."] # [allow (unused)] # [deprecated (since = "0.21.0" , note = "Use Engine::encode_slice")] pub fn encode_engine_slice < E : Engine , T : AsRef < [u8] > > (input : T , output_buf : & mut [u8] , engine : & E ,) -> Result < usize , EncodeSliceError > { engine . encode_slice (input , output_buf) }
    };
}

encode_engine_slice!()
macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl odht :: Config for Config { type Key = Hash64 ; type Value = DefIndex ; type EncodedKey = [u8 ; 8] ; type EncodedValue = [u8 ; 4] ; type H = odht :: UnHashFn ; # [inline] fn encode_key (k : & Hash64) -> [u8 ; 8] { k . as_u64 () . to_le_bytes () } # [inline] fn encode_value (v : & DefIndex) -> [u8 ; 4] { v . as_u32 () . to_le_bytes () } # [inline] fn decode_key (k : & [u8 ; 8]) -> Hash64 { Hash64 :: new (u64 :: from_le_bytes (* k)) } # [inline] fn decode_value (v : & [u8 ; 4]) -> DefIndex { DefIndex :: from_u32 (u32 :: from_le_bytes (* v)) } }
    };
}

impl_80!()
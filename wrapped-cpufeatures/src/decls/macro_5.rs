macro_rules! macro_5 {
    () => {
        # [cfg (any (target_os = "linux" , target_os = "android"))] __expand_check_macro ! { ("aes" , AES) , ("dit" , DIT) , ("sha2" , SHA2) , ("sha3" , SHA3) , ("sm4" , SM4) , }
    };
}

macro_5!()
macro_rules! ContextKey {
    () => {
        # [doc = " An alias to distinguish [`hash_derive_key_context`] outputs from other keys."] pub type ContextKey = [u8 ; KEY_LEN] ;
    };
}

ContextKey!()
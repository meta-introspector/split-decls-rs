// Generated macro for sval_support (module)
macro_rules! Depcrate_kv_keysval_support {
() => {
// Module: crate::kv::key
// Provides: {"sval_support"}
// Dependencies: {}
# [cfg (feature = "kv_sval")] mod sval_support { use super :: * ; use sval :: Value ; use sval_ref :: ValueRef ; impl < 'a > Value for Key < 'a > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S ,) -> sval :: Result { self . key . stream (stream) } } impl < 'a > ValueRef < 'a > for Key < 'a > { fn stream_ref < S : sval :: Stream < 'a > + ? Sized > (& self , stream : & mut S) -> sval :: Result { self . key . stream (stream) } } }
};
}

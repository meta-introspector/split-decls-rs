// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl std :: fmt :: Debug for Stats { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "recv={} sent={} lost={} retrans={}" , self . recv , self . sent , self . lost , self . retrans ,) ? ; write ! (f , " sent_bytes={} recv_bytes={} lost_bytes={}" , self . sent_bytes , self . recv_bytes , self . lost_bytes ,) ? ; Ok (()) } }
};
}

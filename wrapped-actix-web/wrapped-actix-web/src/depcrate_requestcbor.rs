// Generated macro for cbor (module)
macro_rules! Depcrate_requestcbor {
() => {
// Module: crate::request
// Provides: {"cbor"}
// Dependencies: {}
# [cfg (feature = "cbor")] mod cbor { use core :: fmt ; use actix_web :: { ResponseError , http :: StatusCode } ; # [derive (Debug)] pub struct Error (pub serde_cbor :: Error) ; impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . 0) } } impl ResponseError for Error { fn status_code (& self) -> StatusCode { StatusCode :: INTERNAL_SERVER_ERROR } } }
};
}

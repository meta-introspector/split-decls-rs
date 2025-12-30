// Generated macro for error (module)
macro_rules! Depcrate_commiterror {
() => {
// Module: crate::commit
// Provides: {"error"}
// Dependencies: {}
mod error { use std :: { fmt , fmt :: { Debug , Display } , } ; # [doc = " The error returned by various [`commit(…)`][super::Marker::commit()] methods"] # [derive (Debug)] pub struct Error < T : Debug > { # [doc = " The io error that prevented the attempt to succeed"] pub error : std :: io :: Error , # [doc = " The marker or file which was used in the attempt to persist it"] pub instance : T , } impl < T : Debug > Display for Error < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Display :: fmt (& self . error , f) } } impl < T : Debug > std :: error :: Error for Error < T > { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { self . error . source () } } }
};
}

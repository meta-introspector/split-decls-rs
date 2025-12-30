// Generated macro for private (module)
macro_rules! Depcrate_streamprivate {
() => {
// Module: crate::stream
// Provides: {"private"}
// Dependencies: {}
mod private { # [allow (unnameable_types)] pub trait Sealed { } impl < T : Sealed + ? Sized > Sealed for & T { } impl < T : Sealed + ? Sized > Sealed for & mut T { } impl < T : Sealed + ? Sized > Sealed for Box < T > { } impl Sealed for std :: io :: Stdout { } impl Sealed for std :: io :: StdoutLock < '_ > { } impl Sealed for std :: io :: Stderr { } impl Sealed for std :: io :: StderrLock < '_ > { } impl Sealed for dyn std :: io :: Write { } impl Sealed for dyn std :: io :: Write + Send { } impl Sealed for dyn std :: io :: Write + Send + Sync { } impl Sealed for Vec < u8 > { } impl Sealed for std :: fs :: File { } # [allow (deprecated)] impl Sealed for crate :: Buffer { } }
};
}

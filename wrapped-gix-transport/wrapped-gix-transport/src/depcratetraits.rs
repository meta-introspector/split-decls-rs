// Generated macro for traits (module)
macro_rules! Depcratetraits {
() => {
// Module: crate
// Provides: {"traits"}
// Dependencies: {}
mod traits { use std :: convert :: Infallible ; # [doc = " An error which can tell whether it's worth retrying to maybe succeed next time."] pub trait IsSpuriousError : std :: error :: Error { # [doc = " Return `true` if retrying might result in a different outcome due to IO working out differently."] fn is_spurious (& self) -> bool { false } } impl IsSpuriousError for Infallible { } impl IsSpuriousError for std :: io :: Error { fn is_spurious (& self) -> bool { use std :: io :: ErrorKind :: * ; match self . kind () { Unsupported | WriteZero | InvalidInput | InvalidData | WouldBlock | AlreadyExists | AddrNotAvailable | NotConnected | Other | PermissionDenied | NotFound => false , Interrupted | UnexpectedEof | OutOfMemory | TimedOut | BrokenPipe | AddrInUse | ConnectionAborted | ConnectionReset | ConnectionRefused => true , _ => false , } } } }
};
}

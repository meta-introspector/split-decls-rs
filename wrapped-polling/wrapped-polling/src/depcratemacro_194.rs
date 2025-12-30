// Generated macro for macro_194 (macro)
macro_rules! Depcratemacro_194 {
() => {
// Module: crate
// Provides: {"macro_194"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (unix , target_os = "hermit"))] { # [cfg (unix)] use std :: os :: unix :: io :: { AsRawFd , RawFd , AsFd , BorrowedFd } ; # [cfg (target_os = "hermit")] use std :: os :: hermit :: io :: { AsRawFd , RawFd , AsFd , BorrowedFd } ; # [doc = " A resource with a raw file descriptor."] pub trait AsRawSource { # [doc = " Returns the raw file descriptor."] fn raw (& self) -> RawFd ; } impl < T : AsRawFd > AsRawSource for & T { fn raw (& self) -> RawFd { self . as_raw_fd () } } impl AsRawSource for RawFd { fn raw (& self) -> RawFd { * self } } # [doc = " A resource with a borrowed file descriptor."] pub trait AsSource : AsFd { # [doc = " Returns the borrowed file descriptor."] fn source (& self) -> BorrowedFd <'_ > { self . as_fd () } } impl < T : AsFd > AsSource for T { } } else if # [cfg (windows)] { use std :: os :: windows :: io :: { AsRawSocket , RawSocket , AsSocket , BorrowedSocket } ; # [doc = " A resource with a raw socket."] pub trait AsRawSource { # [doc = " Returns the raw socket."] fn raw (& self) -> RawSocket ; } impl < T : AsRawSocket > AsRawSource for & T { fn raw (& self) -> RawSocket { self . as_raw_socket () } } impl AsRawSource for RawSocket { fn raw (& self) -> RawSocket { * self } } # [doc = " A resource with a borrowed socket."] pub trait AsSource : AsSocket { # [doc = " Returns the borrowed socket."] fn source (& self) -> BorrowedSocket <'_ > { self . as_socket () } } impl < T : AsSocket > AsSource for T { } } }
};
}

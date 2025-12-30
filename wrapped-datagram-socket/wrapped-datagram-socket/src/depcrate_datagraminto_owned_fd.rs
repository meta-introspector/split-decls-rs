// Generated macro for into_owned_fd (function)
macro_rules! Depcrate_datagraminto_owned_fd {
() => {
// Module: crate::datagram
// Provides: {"into_owned_fd"}
// Dependencies: {}
# [doc = " `Into<OwnedFd>::into` for types (tokio sockets etc) that don't implement"] # [doc = " `From<OwnedFd>`."] # [cfg (unix)] fn into_owned_fd < F : IntoRawFd > (into_fd : F) -> OwnedFd { unsafe { OwnedFd :: from_raw_fd (into_fd . into_raw_fd ()) } }
};
}

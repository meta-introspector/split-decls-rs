// Generated macro for impl_108 (impl)
macro_rules! Depcrateimpl_108 {
() => {
// Module: crate
// Provides: {"impl_108"}
// Dependencies: {}
# [doc = " Reference types can't be mutated."] # [doc = ""] # [doc = " The worst thing that can happen is that external state is used to change what kind of pointer"] # [doc = " `as_fd()` returns. For instance:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(unix)] {"] # [doc = " use std::cell::Cell;"] # [doc = " use std::net::TcpStream;"] # [doc = " use std::os::unix::io::{AsFd, BorrowedFd};"] # [doc = ""] # [doc = " struct Bar {"] # [doc = "     flag: Cell<bool>,"] # [doc = "     a: TcpStream,"] # [doc = "     b: TcpStream"] # [doc = " }"] # [doc = ""] # [doc = " impl AsFd for Bar {"] # [doc = "     fn as_fd(&self) -> BorrowedFd<'_> {"] # [doc = "         if self.flag.replace(!self.flag.get()) {"] # [doc = "             self.a.as_fd()"] # [doc = "         } else {"] # [doc = "             self.b.as_fd()"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " We solve this problem by only calling `as_fd()` once to get the original source. Implementations"] # [doc = " like this are considered buggy (but not unsound) and are thus not really supported by `async-io`."] unsafe impl < T : ? Sized > IoSafe for & T { }
};
}

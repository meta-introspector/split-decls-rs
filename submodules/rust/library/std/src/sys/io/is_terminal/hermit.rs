// SRC: ../rust/library/std/src/sys/io/is_terminal/hermit.rs
use crate::os::fd::{AsFd, AsRawFd};

pub fn is_terminal(fd: &impl AsFd) -> bool {
    let fd = fd.as_fd();
    hermit_abi::isatty(fd.as_raw_fd())
}

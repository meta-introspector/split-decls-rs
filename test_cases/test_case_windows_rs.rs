// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/net/connection/socket/windows.rs
// Error: expected square brackets
// Problematic line: line 8

use super::{getsockopt, setsockopt, socket_addr_from_c, socket_addr_to_c};
use crate::io::{self, BorrowedBuf, BorrowedCursor, IoSlice, IoSliceMut, Read};
use crate::net::{Shutdown, SocketAddr};
use crate::os::windows::io::{
    AsRawSocket, AsSocket, BorrowedSocket, FromRawSocket, IntoRawSocket, OwnedSocket, RawSocket,
};
use crate::sync::atomic::Atomic;

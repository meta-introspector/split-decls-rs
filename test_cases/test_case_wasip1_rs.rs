// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/net/connection/wasip1.rs
// Error: expected square brackets
// Problematic line: line 14


pub struct Socket(WasiFd);

pub struct TcpStream {
    inner: Socket,
}


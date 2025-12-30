// Generated macro for shutdown (function)
macro_rules! Depcrate_syscalls_socketshutdown {
() => {
// Module: crate::syscalls::socket
// Provides: {"shutdown"}
// Dependencies: {}
fn shutdown (sockfd : i32 , how : i32) -> i32 { let obj = get_object (sockfd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . read () . await . shutdown (how) . await } , None) . map_or_else (| e | - i32 :: from (e) , | () | 0) } ,) }
};
}

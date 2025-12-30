// Generated macro for _connect (function)
macro_rules! Depcrate_pipe_connect {
() => {
// Module: crate::pipe
// Provides: {"_connect"}
// Dependencies: {}
fn _connect (addr : & OsStr) -> io :: Result < File > { let mut r = OpenOptions :: new () ; let mut w = OpenOptions :: new () ; let mut rw = OpenOptions :: new () ; r . read (true) ; w . write (true) ; rw . read (true) . write (true) ; loop { let res = rw . open (addr) . or_else (| _ | r . open (addr)) . or_else (| _ | w . open (addr)) ; match res { Ok (f) => return Ok (f) , Err (ref e) if e . raw_os_error () == Some (ERROR_PIPE_BUSY as i32) => { } Err (e) => return Err (e) , } NamedPipe :: wait (addr , Some (Duration :: new (20 , 0))) ? ; } }
};
}

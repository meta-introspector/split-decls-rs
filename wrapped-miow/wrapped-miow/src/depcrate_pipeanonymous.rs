// Generated macro for anonymous (function)
macro_rules! Depcrate_pipeanonymous {
() => {
// Module: crate::pipe
// Provides: {"anonymous"}
// Dependencies: {}
# [doc = " Creates a new anonymous in-memory pipe, returning the read/write ends of the"] # [doc = " pipe."] # [doc = ""] # [doc = " The buffer size for this pipe may also be specified, but the system will"] # [doc = " normally use this as a suggestion and it's not guaranteed that the buffer"] # [doc = " will be precisely this size."] pub fn anonymous (buffer_size : u32) -> io :: Result < (AnonRead , AnonWrite) > { let mut read = 0 as HANDLE ; let mut write = 0 as HANDLE ; crate :: cvt (unsafe { CreatePipe (& mut read , & mut write , std :: ptr :: null_mut () , buffer_size) }) ? ; Ok ((AnonRead (Handle :: new (read)) , AnonWrite (Handle :: new (write)))) }
};
}

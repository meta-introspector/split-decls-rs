// Generated macro for util (module)
macro_rules! Depcrate_writeutil {
() => {
// Module: crate::write
// Provides: {"util"}
// Dependencies: {}
mod util { pub struct CountBytes < T > { pub count : u32 , pub inner : T , } impl < T > CountBytes < T > where T : std :: io :: Write , { pub fn new (inner : T) -> Self { CountBytes { inner , count : 0 } } } impl < T > std :: io :: Write for CountBytes < T > where T : std :: io :: Write , { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { let written = self . inner . write (buf) ? ; self . count = self . count . checked_add (u32 :: try_from (written) . expect ("we don't write 4GB buffers")) . ok_or_else (| | std :: io :: Error :: other ("Cannot write indices larger than 4 gigabytes")) ? ; Ok (written) } fn flush (& mut self) -> std :: io :: Result < () > { self . inner . flush () } } }
};
}

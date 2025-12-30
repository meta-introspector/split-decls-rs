// Generated macro for impl_499 (impl)
macro_rules! Depcrate_threadimpl_499 {
() => {
// Module: crate::thread
// Provides: {"impl_499"}
// Dependencies: {}
impl Builder { # [doc = " Generates the base configuration for spawning a thread, from which"] # [doc = " configuration methods can be chained."] # [allow (clippy :: new_without_default)] pub fn new () -> Builder { Builder { name : None , stack_size : None , } } # [doc = " Names the thread-to-be. Currently the name is used for identification"] # [doc = " only in panic messages."] pub fn name (mut self , name : String) -> Builder { self . name = Some (name) ; self } # [doc = " Sets the size of the stack (in bytes) for the new thread."] pub fn stack_size (mut self , size : usize) -> Builder { self . stack_size = Some (size) ; self } # [doc = " Spawns a new thread by taking ownership of the `Builder`, and returns an"] # [doc = " `io::Result` to its `JoinHandle`."] # [track_caller] pub fn spawn < F , T > (self , f : F) -> io :: Result < JoinHandle < T > > where F : FnOnce () -> T , F : Send + 'static , T : Send + 'static , { Ok (spawn_internal (f , self . name , self . stack_size , location ! ())) } }
};
}

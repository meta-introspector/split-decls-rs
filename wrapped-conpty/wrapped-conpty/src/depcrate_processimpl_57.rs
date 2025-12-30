// Generated macro for impl_57 (impl)
macro_rules! Depcrate_processimpl_57 {
() => {
// Module: crate::process
// Provides: {"impl_57"}
// Dependencies: {}
impl ProcessOptions { # [doc = " Spawns a new child process inside a new pseudo console."] # [doc = ""] # [doc = " Uses options specified on `self`."] pub fn spawn (& self , command : Command) -> Result < Process , Error > { spawn_command (command , self . console_size) } # [doc = " Specifies the size (x,y) of the new pseudo console window."] # [doc = ""] # [doc = " if set to None, size is inherited from parent console or a"] # [doc = " default value is used."] pub fn set_console_size (& mut self , size_xy : Option < (i16 , i16) >) -> & mut Self { let console_size = size_xy . map (| (x , y) | COORD { X : x , Y : y }) ; self . console_size = console_size ; self } }
};
}

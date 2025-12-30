// Generated macro for LoadCommandIterator (struct)
macro_rules! Depcrate_read_macho_load_commandLoadCommandIterator {
() => {
// Module: crate::read::macho::load_command
// Provides: {"LoadCommandIterator"}
// Dependencies: {}
# [doc = " An iterator for the load commands from a [`MachHeader`]."] # [derive (Debug , Default , Clone , Copy)] pub struct LoadCommandIterator < 'data , E : Endian > { endian : E , data : Bytes < 'data > , ncmds : u32 , }
};
}

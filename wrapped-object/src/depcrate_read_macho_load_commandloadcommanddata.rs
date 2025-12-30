// Generated macro for LoadCommandData (struct)
macro_rules! Depcrate_read_macho_load_commandLoadCommandData {
() => {
// Module: crate::read::macho::load_command
// Provides: {"LoadCommandData"}
// Dependencies: {}
# [doc = " The data for a [`macho::LoadCommand`]."] # [derive (Debug , Clone , Copy)] pub struct LoadCommandData < 'data , E : Endian > { cmd : u32 , data : Bytes < 'data > , marker : PhantomData < E > , }
};
}

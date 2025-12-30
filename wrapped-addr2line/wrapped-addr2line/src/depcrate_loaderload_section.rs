// Generated macro for load_section (function)
macro_rules! Depcrate_loaderload_section {
() => {
// Module: crate::loader
// Provides: {"load_section"}
// Dependencies: {}
fn load_section < 'input , Endian : gimli :: Endianity > (name : Option < & 'static str > , file : & object :: File < 'input > , endian : Endian , arena_data : & 'input Arena < Vec < u8 > > ,) -> Result < gimli :: EndianSlice < 'input , Endian > > { let data = match name . and_then (| name | file . section_by_name (name)) { Some (section) => match section . uncompressed_data () ? { Cow :: Borrowed (b) => b , Cow :: Owned (b) => arena_data . alloc (b) , } , None => & [] , } ; Ok (gimli :: EndianSlice :: new (data , endian)) }
};
}

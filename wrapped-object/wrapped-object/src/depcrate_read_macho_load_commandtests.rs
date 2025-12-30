// Generated macro for tests (module)
macro_rules! Depcrate_read_macho_load_commandtests {
() => {
// Module: crate::read::macho::load_command
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: LittleEndian ; # [test] fn cmd_size_invalid () { # [repr (align (16))] struct Align < const N : usize > ([u8 ; N]) ; let mut commands = LoadCommandIterator :: new (LittleEndian , & Align ([0 ; 8]) . 0 , 10) ; assert ! (commands . next () . is_err ()) ; let mut commands = LoadCommandIterator :: new (LittleEndian , & Align ([0 , 0 , 0 , 0 , 7 , 0 , 0 , 0 , 0]) . 0 , 10) ; assert ! (commands . next () . is_err ()) ; let mut commands = LoadCommandIterator :: new (LittleEndian , & Align ([0 , 0 , 0 , 0 , 8 , 0 , 0 , 0 , 0]) . 0 , 10) ; assert ! (commands . next () . is_ok ()) ; } # [test] fn function_starts_invalid_uleb128 () { use crate :: endian :: U32 ; use crate :: macho ; let data = [0x80] ; let cmd = macho :: LinkeditDataCommand { cmd : U32 :: new (LittleEndian , macho :: LC_FUNCTION_STARTS) , cmdsize : U32 :: new (LittleEndian , 16) , dataoff : U32 :: new (LittleEndian , 0) , datasize : U32 :: new (LittleEndian , data . len () as u32) , } ; let mut iter = cmd . function_starts (LittleEndian , & data [..] , 0) . unwrap () ; assert ! (iter . next () . is_err ()) ; assert ! (iter . next () . transpose () . is_none ()) ; } }
};
}

// Generated macro for Write (trait)
macro_rules! Depcrate_writer_file_helpersWrite {
() => {
// Module: crate::writer::file::helpers
// Provides: {"Write"}
// Dependencies: {}
pub trait Write { fn write_header < T : Sized > (& mut self , value : & T) ; fn write_u16 (& mut self , value : u16) ; fn write_u32 (& mut self , value : u32) ; fn write_u64 (& mut self , value : u64) ; fn write_code (& mut self , value : u32 , size : usize) ; fn write_index (& mut self , index : u32 , len : usize) ; fn write_compressed (& mut self , value : usize) ; fn write_value (& mut self , value : & Value) ; }
};
}

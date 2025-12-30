// Generated macro for impl_24 (impl)
macro_rules! Depcrate_messageimpl_24 {
() => {
// Module: crate::message
// Provides: {"impl_24"}
// Dependencies: {}
impl MessageReader { pub fn new () -> Self { MessageReader { storage : vec ! [0u8 ; 256] , read_bytes : 0 , total_size : None , } } pub fn clear (& mut self) { if self . storage . capacity () < 256 { self . storage = vec ! [0u8 ; 256] ; } else { self . storage . clear () ; self . storage . resize (256 , 0) ; } self . read_bytes = 0 ; self . total_size = None ; } pub fn get_buf (& mut self) -> & mut [u8] { if let Some (ts) = self . total_size { & mut self . storage [self . read_bytes .. ts] } else { & mut self . storage [self . read_bytes .. FIXED_HEADER_SIZE] } } pub fn buf_written_to (& mut self , count : usize) -> Result < Option < Vec < u8 > > , DemarshalError > { self . read_bytes += count ; if self . total_size . is_none () && self . read_bytes >= FIXED_HEADER_SIZE { let start = message_start_parse (& self . storage) ? ; self . total_size = Some (start . total_size) ; self . storage . resize (start . total_size , 0) ; } if Some (self . read_bytes) == self . total_size { let r = std :: mem :: replace (& mut self . storage , vec ! ()) ; assert_eq ! (r . len () , self . read_bytes) ; self . clear () ; Ok (Some (r)) } else { Ok (None) } } pub fn block_until_next_message < R : std :: io :: Read > (& mut self , reader : & mut R) -> Result < Vec < u8 > , std :: io :: Error > { loop { let buflen = { let buf = self . get_buf () ; reader . read_exact (buf) ? ; buf . len () } ; if let Some (v) = self . buf_written_to (buflen) . map_err (| e | std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidData , e)) ? { return Ok (v) ; } } ; } }
};
}

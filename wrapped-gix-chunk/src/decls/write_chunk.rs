macro_rules! deps {
    () => {
        Id!();
        Entry!();
    };
}

macro_rules! write_chunk {
    () => {
        deps!();
        mod write_chunk { use std :: collections :: VecDeque ; use crate :: file :: index ; # [doc = " A [`Write`][std::io::Write] implementation that validates chunk sizes while allowing the user to know"] # [doc = " which chunk is to be written next."] pub struct Chunk < W > { chunks_to_write : VecDeque < index :: Entry > , inner : W , next_chunk : Option < index :: Entry > , written_bytes : usize , } impl < W > Chunk < W > where W : std :: io :: Write , { pub (crate) fn new (out : W , chunks : VecDeque < index :: Entry >) -> Chunk < W > where W : std :: io :: Write , { Chunk { chunks_to_write : chunks , inner : out , next_chunk : None , written_bytes : 0 , } } } impl < W > std :: io :: Write for Chunk < W > where W : std :: io :: Write , { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { let written = self . inner . write (buf) ? ; self . written_bytes += written ; Ok (written) } fn flush (& mut self) -> std :: io :: Result < () > { self . inner . flush () } } impl < W > Chunk < W > { # [doc = " Return the inner writer - should only be called once there is no more chunk to write."] pub fn into_inner (self) -> W { self . inner } # [doc = " Return the next chunk-id to write, if there is one."] pub fn next_chunk (& mut self) -> Option < crate :: Id > { if let Some (entry) = self . next_chunk . take () { assert_eq ! (entry . offset . end , self . written_bytes as u64 , "BUG: expected to write {} bytes, but only wrote {} for chunk {:?}" , entry . offset . end , self . written_bytes , std :: str :: from_utf8 (& entry . kind)) ; } self . written_bytes = 0 ; self . next_chunk = self . chunks_to_write . pop_front () ; self . next_chunk . as_ref () . map (| e | e . kind) } } }
    };
}

write_chunk!()
macro_rules! deps {
    () => {
        Lzma2Reader!();
        Read!();
        Result!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < R : Read > Read for Lzma2Reader < R > { fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { if buf . is_empty () { return Ok (0) ; } if self . end_reached { return Ok (0) ; } let mut size = 0 ; let mut len = buf . len () ; let mut off = 0 ; while len > 0 { if self . uncompressed_size == 0 { self . decode_chunk_header () ? ; if self . end_reached { return Ok (size) ; } } let copy_size_max = self . uncompressed_size . min (len) ; if ! self . is_lzma_chunk { self . lz . copy_uncompressed (& mut self . inner , copy_size_max) ? ; } else { self . lz . set_limit (copy_size_max) ; if let Some (lzma) = self . lzma . as_mut () { lzma . decode (& mut self . lz , & mut self . rc) ? ; } } { let copied_size = self . lz . flush (buf , off) ? ; off = off . saturating_add (copied_size) ; len = len . saturating_sub (copied_size) ; size = size . saturating_add (copied_size) ; self . uncompressed_size = self . uncompressed_size . saturating_sub (copied_size) ; if self . uncompressed_size == 0 && (! self . rc . is_finished () || self . lz . has_pending ()) { return Err (error_invalid_input ("rc not finished or lz has pending")) ; } } } Ok (size) } }
    };
}

impl_81!()
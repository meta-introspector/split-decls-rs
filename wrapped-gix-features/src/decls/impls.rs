macro_rules! deps {
    () => {
        FlushCompress!();
        Status!();
        Write!();
        Compress!();
    };
}

macro_rules! impls {
    () => {
        deps!();
        mod impls { use std :: io ; use crate :: zlib :: stream :: deflate :: { self , Compress , FlushCompress } ; use crate :: zlib :: Status ; pub (crate) fn new_compress () -> Compress { Compress :: new () } impl < W > deflate :: Write < W > where W : io :: Write , { # [doc = " Create a new instance writing compressed bytes to `inner`."] pub fn new (inner : W) -> deflate :: Write < W > { deflate :: Write { compressor : new_compress () , inner , buf : [0 ; deflate :: BUF_SIZE] , } } # [doc = " Reset the compressor, starting a new compression stream."] # [doc = ""] # [doc = " That way multiple streams can be written to the same inner writer."] pub fn reset (& mut self) { self . compressor . reset () ; } # [doc = " Consume `self` and return the inner writer."] pub fn into_inner (self) -> W { self . inner } fn write_inner (& mut self , mut buf : & [u8] , flush : FlushCompress) -> io :: Result < usize > { let total_in_when_start = self . compressor . total_in () ; loop { let last_total_in = self . compressor . total_in () ; let last_total_out = self . compressor . total_out () ; let status = self . compressor . compress (buf , & mut self . buf , flush) . map_err (io :: Error :: other) ? ; let written = self . compressor . total_out () - last_total_out ; if written > 0 { self . inner . write_all (& self . buf [.. written as usize]) ? ; } match status { Status :: StreamEnd => return Ok ((self . compressor . total_in () - total_in_when_start) as usize) , Status :: Ok | Status :: BufError => { let consumed = self . compressor . total_in () - last_total_in ; buf = & buf [consumed as usize ..] ; if self . compressor . total_out () > last_total_out { continue ; } if self . compressor . total_in () > last_total_in { continue ; } return Ok ((self . compressor . total_in () - total_in_when_start) as usize) ; } } } } } impl < W : io :: Write > io :: Write for deflate :: Write < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . write_inner (buf , FlushCompress :: None) } fn flush (& mut self) -> io :: Result < () > { self . write_inner (& [] , FlushCompress :: Finish) . map (| _ | ()) } } }
    };
}

impls!()
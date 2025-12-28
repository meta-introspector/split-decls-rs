macro_rules! deps {
    () => {
        RangeDecoder!();
        LzmaDecoder!();
        LzDecoder!();
    };
}

macro_rules! LzmaReader {
    () => {
        deps!();
        # [doc = " A single-threaded LZMA decompressor."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use std::io::Read;"] # [doc = ""] # [doc = " use lzma_rust2::LzmaReader;"] # [doc = ""] # [doc = " let compressed: Vec<u8> = vec!["] # [doc = "     93, 0, 0, 128, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 36, 25, 73, 152, 111, 22, 2,"] # [doc = "     140, 232, 230, 91, 177, 71, 198, 206, 183, 99, 255, 255, 60, 172, 0, 0,"] # [doc = " ];"] # [doc = " let mut reader = LzmaReader::new_mem_limit(compressed.as_slice(), u32::MAX, None).unwrap();"] # [doc = " let mut buf = [0; 1024];"] # [doc = " let mut out = Vec::new();"] # [doc = " loop {"] # [doc = "     let n = reader.read(&mut buf).unwrap();"] # [doc = "     if n == 0 {"] # [doc = "         break;"] # [doc = "     }"] # [doc = "     out.extend_from_slice(&buf[..n]);"] # [doc = " }"] # [doc = " assert_eq!(out, b\"Hello, world!\");"] # [doc = " ```"] pub struct LzmaReader < R > { lz : LzDecoder , rc : RangeDecoder < R > , lzma : LzmaDecoder , end_reached : bool , relaxed_end_cond : bool , remaining_size : u64 , }
    };
}

LzmaReader!();
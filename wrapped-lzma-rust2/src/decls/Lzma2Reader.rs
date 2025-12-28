macro_rules! deps {
    () => {
        RangeDecoder!();
        LzDecoder!();
        RangeDecoderBuffer!();
        LzmaOptions!();
        LzmaDecoder!();
    };
}

macro_rules! Lzma2Reader {
    () => {
        deps!();
        # [doc = " A single-threaded LZMA2 decompressor."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use std::io::Read;"] # [doc = ""] # [doc = " use lzma_rust2::{Lzma2Reader, LzmaOptions};"] # [doc = ""] # [doc = " let compressed: Vec<u8> = vec!["] # [doc = "     1, 0, 12, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 0,"] # [doc = " ];"] # [doc = " let mut reader = Lzma2Reader::new(compressed.as_slice(), LzmaOptions::DICT_SIZE_DEFAULT, None);"] # [doc = " let mut decompressed = Vec::new();"] # [doc = " reader.read_to_end(&mut decompressed).unwrap();"] # [doc = " assert_eq!(&decompressed[..], b\"Hello, world!\");"] # [doc = " ```"] pub struct Lzma2Reader < R > { inner : R , lz : LzDecoder , rc : RangeDecoder < RangeDecoderBuffer > , lzma : Option < LzmaDecoder > , uncompressed_size : usize , is_lzma_chunk : bool , need_dict_reset : bool , need_props : bool , end_reached : bool , }
    };
}

Lzma2Reader!()
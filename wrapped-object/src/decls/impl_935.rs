macro_rules! deps {
    () => {
        CompressedData!();
        Error!();
        Result!();
        CompressionFormat!();
    };
}

macro_rules! impl_935 {
    () => {
        deps!();
        impl < 'data > CompressedData < 'data > { # [doc = " Data that is uncompressed."] # [inline] pub fn none (data : & 'data [u8]) -> Self { CompressedData { format : CompressionFormat :: None , data , uncompressed_size : data . len () as u64 , } } # [doc = " Return the uncompressed data."] # [doc = ""] # [doc = " Returns an error for invalid data or unsupported compression."] # [doc = " This includes if the data is compressed but the `compression` feature"] # [doc = " for this crate is disabled."] pub fn decompress (self) -> Result < Cow < 'data , [u8] > > { match self . format { CompressionFormat :: None => Ok (Cow :: Borrowed (self . data)) , # [cfg (feature = "compression")] CompressionFormat :: Zlib | CompressionFormat :: Zstandard => { use core :: convert :: TryInto ; use std :: io :: Read ; let size = self . uncompressed_size . try_into () . ok () . read_error ("Uncompressed data size is too large.") ? ; let mut decompressed = Vec :: new () ; decompressed . try_reserve_exact (size) . ok () . read_error ("Uncompressed data allocation failed") ? ; match self . format { CompressionFormat :: Zlib => { let mut decompress = flate2 :: Decompress :: new (true) ; decompress . decompress_vec (self . data , & mut decompressed , flate2 :: FlushDecompress :: Finish ,) . ok () . read_error ("Invalid zlib compressed data") ? ; } CompressionFormat :: Zstandard => { let mut input = self . data ; while ! input . is_empty () { let mut decoder = match ruzstd :: decoding :: StreamingDecoder :: new (& mut input) { Ok (decoder) => decoder , Err (ruzstd :: decoding :: errors :: FrameDecoderError :: ReadFrameHeaderError (ruzstd :: decoding :: errors :: ReadFrameHeaderError :: SkipFrame { length , .. } ,) ,) => { input = input . get (length as usize ..) . read_error ("Invalid zstd compressed data") ? ; continue ; } x => x . ok () . read_error ("Invalid zstd compressed data") ? , } ; decoder . read_to_end (& mut decompressed) . ok () . read_error ("Invalid zstd compressed data") ? ; } } _ => unreachable ! () , } if size != decompressed . len () { return Err (Error ("Uncompressed data size does not match compression header" ,)) ; } Ok (Cow :: Owned (decompressed)) } _ => Err (Error ("Unsupported compressed data.")) , } } }
    };
}

impl_935!();
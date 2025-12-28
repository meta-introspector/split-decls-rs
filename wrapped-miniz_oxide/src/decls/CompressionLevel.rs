macro_rules! CompressionLevel {
    () => {
        # [doc = " How much processing the compressor should do to compress the data."] # [doc = " `NoCompression` and `Bestspeed` have special meanings, the other levels determine the number"] # [doc = " of checks for matches in the hash chains and whether to use lazy or greedy parsing."] # [repr (i32)] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum CompressionLevel { # [doc = " Don't do any compression, only output uncompressed blocks."] NoCompression = 0 , # [doc = " Fast compression. Uses a special compression routine that is optimized for speed."] BestSpeed = 1 , # [doc = " Slow/high compression. Do a lot of checks to try to find good matches."] BestCompression = 9 , # [doc = " Even more checks, can be very slow."] UberCompression = 10 , # [doc = " Default compromise between speed and compression."] DefaultLevel = 6 , # [doc = " Use the default compression level."] DefaultCompression = - 1 , }
    };
}

CompressionLevel!()
macro_rules! NUM_PROBES {
    () => {
        # [doc = " The maximum number of checks for matches in the hash table the compressor will make for each"] # [doc = " compression level."] pub (crate) const NUM_PROBES : [u16 ; 11] = [0 , 1 , 6 , 32 , 16 , 32 , 128 , 256 , 512 , 768 , 1500] ;
    };
}

NUM_PROBES!();
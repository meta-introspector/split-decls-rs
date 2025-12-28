macro_rules! deps {
    () => {
        Endian!();
        Header!();
        U32!();
    };
}

macro_rules! GnuHashHeader {
    () => {
        deps!();
        # [doc = " Header of `SHT_GNU_HASH` section."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct GnuHashHeader < E : Endian > { # [doc = " The number of hash buckets."] pub bucket_count : U32 < E > , # [doc = " The symbol table index of the first symbol in the hash."] pub symbol_base : U32 < E > , # [doc = " The number of words in the bloom filter."] # [doc = ""] # [doc = " Must be a non-zero power of 2."] pub bloom_count : U32 < E > , # [doc = " The bit shift count for the bloom filter."] pub bloom_shift : U32 < E > , }
    };
}

GnuHashHeader!();
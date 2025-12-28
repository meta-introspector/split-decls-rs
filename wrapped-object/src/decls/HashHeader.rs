macro_rules! deps {
    () => {
        Header!();
        Endian!();
        U32!();
    };
}

macro_rules! HashHeader {
    () => {
        deps!();
        # [doc = " Header of `SHT_HASH` section."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct HashHeader < E : Endian > { # [doc = " The number of hash buckets."] pub bucket_count : U32 < E > , # [doc = " The number of chain values."] pub chain_count : U32 < E > , }
    };
}

HashHeader!();
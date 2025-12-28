macro_rules! deps {
    () => {
        Chunk!();
        ChunkSize!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Chunk { # [cfg (test)] fn assert_valid (& self , chunk_domain_size : ChunkSize) { assert ! (chunk_domain_size as usize <= CHUNK_BITS) ; match * self { Zeros | Ones => { } Mixed (count , ref words) => { assert ! (0 < count && count < chunk_domain_size) ; assert_eq ! (words . iter () . map (| w | w . count_ones () as ChunkSize) . sum ::< ChunkSize > () , count) ; let num_words = num_words (chunk_domain_size as usize) ; if num_words < CHUNK_WORDS { assert_eq ! (words [num_words ..] . iter () . map (| w | w . count_ones () as ChunkSize) . sum ::< ChunkSize > () , 0) ; } } } } # [doc = " Count the number of 1s in the chunk."] fn count (& self , chunk_domain_size : ChunkSize) -> usize { match * self { Zeros => 0 , Ones => chunk_domain_size as usize , Mixed (count , _) => count as usize , } } }
    };
}

impl_33!();
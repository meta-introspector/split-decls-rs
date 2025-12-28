macro_rules! deps {
    () => {
        FilterType!();
    };
}

macro_rules! BlockHeader {
    () => {
        deps!();
        # [derive (Debug)] struct BlockHeader { header_size : usize , compressed_size : Option < u64 > , uncompressed_size : Option < u64 > , filters : [Option < FilterType > ; 4] , properties : [u32 ; 4] , }
    };
}

BlockHeader!();
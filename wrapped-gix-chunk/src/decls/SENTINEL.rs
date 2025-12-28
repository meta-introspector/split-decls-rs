macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! SENTINEL {
    () => {
        deps!();
        # [doc = " A special value denoting the end of the chunk file table of contents."] pub const SENTINEL : Id = [0u8 ; 4] ;
    };
}

SENTINEL!()
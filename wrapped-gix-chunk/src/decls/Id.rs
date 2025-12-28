macro_rules! Id {
    () => {
        # [doc = " An identifier to describe the kind of chunk, unique within a chunk file, typically in ASCII"] pub type Id = [u8 ; 4] ;
    };
}

Id!();
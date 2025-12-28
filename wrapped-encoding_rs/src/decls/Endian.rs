macro_rules! Endian {
    () => {
        pub trait Endian { const OPPOSITE_ENDIAN : bool ; }
    };
}

Endian!()
macro_rules! deps {
    () => {
        Reader!();
        ReaderOffset!();
        Location!();
    };
}

macro_rules! impl_507 {
    () => {
        deps!();
        impl < R , Offset > Location < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " Return true if the piece is empty."] pub fn is_empty (& self) -> bool { matches ! (* self , Location :: Empty) } }
    };
}

impl_507!()
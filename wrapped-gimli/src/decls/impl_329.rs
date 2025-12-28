macro_rules! deps {
    () => {
        Relocate!();
        RelocateReader!();
        Reader!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < R , T > RelocateReader < R , T > where R : Reader < Offset = usize > , T : Relocate < R :: Offset > , { # [doc = " Create a new `RelocateReader` which applies relocations to the given section reader."] pub fn new (section : R , relocate : T) -> Self { let reader = section . clone () ; Self { section , reader , relocate , } } }
    };
}

impl_329!();
macro_rules! deps {
    () => {
        Reader!();
        Relocate!();
    };
}

macro_rules! RelocateReader {
    () => {
        deps!();
        # [doc = " A `Reader` which applies relocations to addresses and offsets."] # [doc = ""] # [doc = " This is useful for reading sections which contain relocations,"] # [doc = " such as those in a relocatable object file."] # [doc = " It is generally not used for reading sections in an executable file."] # [derive (Debug , Clone)] pub struct RelocateReader < R : Reader < Offset = usize > , T : Relocate < R :: Offset > > { section : R , reader : R , relocate : T , }
    };
}

RelocateReader!();
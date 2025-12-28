macro_rules! Class {
    () => {
        # [doc = " An ELF file class."] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq)] pub struct Class { # [doc = " Whether the file is 64-bit."] pub is_64 : bool , }
    };
}

Class!()
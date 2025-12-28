macro_rules! Format {
    () => {
        # [doc = " Whether the format of a compilation unit is 32- or 64-bit."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum Format { # [doc = " 64-bit DWARF"] Dwarf64 = 8 , # [doc = " 32-bit DWARF"] Dwarf32 = 4 , }
    };
}

Format!()
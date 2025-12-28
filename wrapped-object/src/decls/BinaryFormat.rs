macro_rules! deps {
    () => {
        MachO!();
    };
}

macro_rules! BinaryFormat {
    () => {
        deps!();
        # [doc = " A binary file format."] # [allow (missing_docs)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum BinaryFormat { Coff , Elf , MachO , Pe , Wasm , Xcoff , }
    };
}

BinaryFormat!()
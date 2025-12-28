macro_rules! deps {
    () => {
        WasmFile!();
        SectionHeader!();
        ObjectSection!();
    };
}

macro_rules! WasmSection {
    () => {
        deps!();
        # [doc = " A section in a [`WasmFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct WasmSection < 'data , 'file , R = & 'data [u8] > { file : & 'file WasmFile < 'data , R > , section : & 'file SectionHeader < 'data > , }
    };
}

WasmSection!()
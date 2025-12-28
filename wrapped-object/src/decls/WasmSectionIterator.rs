macro_rules! deps {
    () => {
        SectionHeader!();
        WasmFile!();
    };
}

macro_rules! WasmSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`WasmFile`]."] # [derive (Debug)] pub struct WasmSectionIterator < 'data , 'file , R = & 'data [u8] > { file : & 'file WasmFile < 'data , R > , sections : slice :: Iter < 'file , SectionHeader < 'data > > , }
    };
}

WasmSectionIterator!()
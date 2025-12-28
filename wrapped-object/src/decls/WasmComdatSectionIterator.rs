macro_rules! deps {
    () => {
        WasmFile!();
    };
}

macro_rules! WasmComdatSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`WasmFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct WasmComdatSectionIterator < 'data , 'file , R = & 'data [u8] > { # [allow (unused)] file : & 'file WasmFile < 'data , R > , }
    };
}

WasmComdatSectionIterator!()
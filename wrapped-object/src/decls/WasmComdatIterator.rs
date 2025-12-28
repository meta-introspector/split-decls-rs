macro_rules! deps {
    () => {
        WasmFile!();
    };
}

macro_rules! WasmComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`WasmFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct WasmComdatIterator < 'data , 'file , R = & 'data [u8] > { # [allow (unused)] file : & 'file WasmFile < 'data , R > , }
    };
}

WasmComdatIterator!()
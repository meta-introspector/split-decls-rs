macro_rules! deps {
    () => {
        WasmFile!();
    };
}

macro_rules! WasmSegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the segments in a [`WasmFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct WasmSegmentIterator < 'data , 'file , R = & 'data [u8] > { # [allow (unused)] file : & 'file WasmFile < 'data , R > , }
    };
}

WasmSegmentIterator!()
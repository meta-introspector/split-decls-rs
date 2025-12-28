macro_rules! deps {
    () => {
        WasmFile!();
    };
}

macro_rules! WasmSegment {
    () => {
        deps!();
        # [doc = " A segment in a [`WasmFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct WasmSegment < 'data , 'file , R = & 'data [u8] > { # [allow (unused)] file : & 'file WasmFile < 'data , R > , }
    };
}

WasmSegment!();
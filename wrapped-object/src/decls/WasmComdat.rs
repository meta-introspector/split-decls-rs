macro_rules! deps {
    () => {
        WasmFile!();
    };
}

macro_rules! WasmComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`WasmFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct WasmComdat < 'data , 'file , R = & 'data [u8] > { # [allow (unused)] file : & 'file WasmFile < 'data , R > , }
    };
}

WasmComdat!();
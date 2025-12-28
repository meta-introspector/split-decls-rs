macro_rules! deps {
    () => {
        WasmFile!();
    };
}

macro_rules! NativeFile {
    () => {
        deps!();
        # [doc = " The native executable file for the target platform."] # [cfg (all (feature = "wasm" , target_arch = "wasm32" , feature = "wasm"))] pub type NativeFile < 'data , R = & 'data [u8] > = wasm :: WasmFile < 'data , R > ;
    };
}

NativeFile!();
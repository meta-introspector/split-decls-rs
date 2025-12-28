macro_rules! CallingConvention {
    () => {
        # [doc (hidden)] pub enum CallingConvention { Stdcall (usize) , Cdecl , }
    };
}

CallingConvention!()
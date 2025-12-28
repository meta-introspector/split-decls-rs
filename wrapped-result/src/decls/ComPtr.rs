macro_rules! ComPtr {
    () => {
        # [repr (transparent)] pub struct ComPtr (core :: ptr :: NonNull < core :: ffi :: c_void >) ;
    };
}

ComPtr!()
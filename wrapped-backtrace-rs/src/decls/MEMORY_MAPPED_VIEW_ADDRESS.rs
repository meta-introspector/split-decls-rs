macro_rules! MEMORY_MAPPED_VIEW_ADDRESS {
    () => {
        # [repr (C)] # [derive (Clone , Copy)] pub struct MEMORY_MAPPED_VIEW_ADDRESS { pub Value : * mut core :: ffi :: c_void , }
    };
}

MEMORY_MAPPED_VIEW_ADDRESS!()
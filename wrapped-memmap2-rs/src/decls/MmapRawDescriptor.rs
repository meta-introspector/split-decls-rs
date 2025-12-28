macro_rules! MmapRawDescriptor {
    () => {
        # [cfg (windows)] pub struct MmapRawDescriptor (RawHandle) ;
    };
}

MmapRawDescriptor!();
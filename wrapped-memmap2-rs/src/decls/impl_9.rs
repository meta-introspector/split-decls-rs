macro_rules! deps {
    () => {
        MmapRawDescriptor!();
        MmapAsRawDesc!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [cfg (windows)] impl MmapAsRawDesc for RawHandle { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (* self) } }
    };
}

impl_9!()
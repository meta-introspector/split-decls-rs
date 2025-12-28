macro_rules! deps {
    () => {
        MmapAsRawDesc!();
        MmapRawDescriptor!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (windows)] impl MmapAsRawDesc for RawHandle { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (* self) } }
    };
}

impl_14!();
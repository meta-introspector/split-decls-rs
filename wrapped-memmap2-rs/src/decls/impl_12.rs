macro_rules! deps {
    () => {
        MmapAsRawDesc!();
        MmapRawDescriptor!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (unix)] impl MmapAsRawDesc for RawFd { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (* self) } }
    };
}

impl_12!()
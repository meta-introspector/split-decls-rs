macro_rules! deps {
    () => {
        MmapAsRawDesc!();
        MmapRawDescriptor!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [cfg (not (any (unix , windows)))] impl MmapAsRawDesc for & File { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (self) } }
    };
}

impl_11!();
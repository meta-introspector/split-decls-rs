macro_rules! deps {
    () => {
        MmapAsRawDesc!();
        MmapRawDescriptor!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (windows)] impl < T > MmapAsRawDesc for & T where T : AsRawHandle , { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (self . as_raw_handle ()) } }
    };
}

impl_10!()
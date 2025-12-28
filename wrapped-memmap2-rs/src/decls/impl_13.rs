macro_rules! deps {
    () => {
        MmapAsRawDesc!();
        MmapRawDescriptor!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (unix)] impl < T > MmapAsRawDesc for & T where T : AsRawFd , { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (self . as_raw_fd ()) } }
    };
}

impl_13!();
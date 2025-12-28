macro_rules! deps {
    () => {
        MmapRawDescriptor!();
    };
}

macro_rules! MmapAsRawDesc {
    () => {
        deps!();
        pub trait MmapAsRawDesc { fn as_raw_desc (& self) -> MmapRawDescriptor ; }
    };
}

MmapAsRawDesc!()
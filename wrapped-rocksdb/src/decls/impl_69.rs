macro_rules! deps {
    () => {
        Decision!();
        CompactionFilterCallback!();
        CompactionFilterFn!();
        CompactionFilter!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < F > CompactionFilter for CompactionFilterCallback < F > where F : CompactionFilterFn , { fn name (& self) -> & CStr { self . name . as_c_str () } fn filter (& mut self , level : u32 , key : & [u8] , value : & [u8]) -> Decision { (self . filter_fn) (level , key , value) } }
    };
}

impl_69!()
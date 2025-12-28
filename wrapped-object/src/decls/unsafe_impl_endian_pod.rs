macro_rules! deps {
    () => {
        Endian!();
        Pod!();
    };
}

macro_rules! unsafe_impl_endian_pod {
    () => {
        deps!();
        macro_rules ! unsafe_impl_endian_pod { ($ ($ struct_name : ident) ,+ $ (,) ?) => { $ (unsafe impl < E : Endian > Pod for $ struct_name < E > { }) + } }
    };
}

unsafe_impl_endian_pod!()
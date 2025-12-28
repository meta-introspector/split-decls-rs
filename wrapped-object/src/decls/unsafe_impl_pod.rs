macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! unsafe_impl_pod {
    () => {
        deps!();
        macro_rules ! unsafe_impl_pod { ($ ($ struct_name : ident) ,+ $ (,) ?) => { $ (unsafe impl Pod for $ struct_name { }) + } }
    };
}

unsafe_impl_pod!();
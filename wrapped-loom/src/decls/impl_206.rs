macro_rules! deps {
    () => {
        Cell!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < T : PartialEq + Copy > PartialEq for Cell < T > { fn eq (& self , other : & Self) -> bool { self . get () == other . get () } }
    };
}

impl_206!()
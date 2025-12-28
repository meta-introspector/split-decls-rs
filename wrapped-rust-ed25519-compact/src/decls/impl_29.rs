macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl PartialEq for Fe { fn eq (& self , other : & Fe) -> bool { let & Fe (self_elems) = self ; let & Fe (other_elems) = other ; self_elems == other_elems } }
    };
}

impl_29!();
macro_rules! deps {
    () => {
        IndexAttr!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl IndexAttr { fn const_from_lit (& self , lit : & Lit) -> isize { if let Lit :: Int (ref n) = lit { n . base10_parse () . expect ("invalid value") } else { panic ! ("unexpected value") } } }
    };
}

impl_13!()
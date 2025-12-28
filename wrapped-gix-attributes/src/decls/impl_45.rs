macro_rules! deps {
    () => {
        Outcome!();
        Match!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Match { fn to_outer < 'a > (& self , out : & 'a Outcome) -> crate :: search :: Match < 'a > { crate :: search :: Match { pattern : out . patterns . resolve (self . pattern) . expect ("pattern still present") , assignment : out . assignments . resolve (self . assignment) . expect ("assignment present") . as_ref () , kind : self . kind , location : self . location . to_outer (out) , } } }
    };
}

impl_45!()
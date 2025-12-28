macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        # [allow (clippy :: non_canonical_partial_ord_impl)] impl < 'i > PartialOrd for Position < 'i > { fn partial_cmp (& self , other : & Position < 'i >) -> Option < Ordering > { if ptr :: eq (self . input , other . input) { self . pos . partial_cmp (& other . pos) } else { None } } }
    };
}

impl_121!();
macro_rules! deps {
    () => {
        Equivalent!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < Q : ? Sized , K : ? Sized > Equivalent < K > for Q where Q : Eq , K : Borrow < Q > , { # [inline] fn equivalent (& self , key : & K) -> bool { PartialEq :: eq (self , key . borrow ()) } }
    };
}

impl_1!();
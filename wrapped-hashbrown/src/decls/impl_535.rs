macro_rules! deps {
    () => {
        Equivalent!();
    };
}

macro_rules! impl_535 {
    () => {
        deps!();
        # [cfg (not (feature = "equivalent"))] impl < Q : ? Sized , K : ? Sized > Equivalent < K > for Q where Q : Eq , K : core :: borrow :: Borrow < Q > , { fn equivalent (& self , key : & K) -> bool { self == key . borrow () } }
    };
}

impl_535!()
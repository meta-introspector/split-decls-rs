macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Extend < (String , String) > for Env { fn extend < T : IntoIterator < Item = (String , String) > > (& mut self , iter : T) { self . entries . extend (iter) ; } }
    };
}

impl_54!();
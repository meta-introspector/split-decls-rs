macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl FromIterator < (String , String) > for Env { fn from_iter < T : IntoIterator < Item = (String , String) > > (iter : T) -> Self { Env { entries : FromIterator :: from_iter (iter) } } }
    };
}

impl_55!();
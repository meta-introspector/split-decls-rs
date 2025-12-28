macro_rules! deps {
    () => {
        Value!();
        Map!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl FromIterator < (String , Value) > for Map < String , Value > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = (String , Value) > , { Map { map : FromIterator :: from_iter (iter) , } } }
    };
}

impl_90!();
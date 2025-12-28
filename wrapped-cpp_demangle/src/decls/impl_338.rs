macro_rules! deps {
    () => {
        SubstitutionTable!();
        Substitutable!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl FromIterator < Substitutable > for SubstitutionTable { fn from_iter < I : IntoIterator < Item = Substitutable > > (iter : I) -> Self { SubstitutionTable { substitutions : Vec :: from_iter (iter) , non_substitutions : vec ! [] , } } }
    };
}

impl_338!();
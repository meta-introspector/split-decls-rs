macro_rules! deps {
    () => {
        SubstitutionTable!();
        Result!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl fmt :: Debug for SubstitutionTable { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . pad ("SubstitutionTable ") ? ; f . debug_map () . entries (self . substitutions . iter () . enumerate ()) . finish () ? ; f . pad (" non_substitutions ") ? ; f . debug_map () . entries (self . non_substitutions . iter () . enumerate ()) . finish () } }
    };
}

impl_336!()
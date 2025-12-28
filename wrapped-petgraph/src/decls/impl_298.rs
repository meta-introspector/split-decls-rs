macro_rules! deps {
    () => {
        IndexType!();
        EdgeReferences!();
        NoPretty!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < E , Ix > fmt :: Debug for EdgeReferences < '_ , E , Ix > where E : fmt :: Debug , Ix : IndexType , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut edge_list = f . debug_list () ; let iter : Self = self . clone () ; for e in iter { if core :: mem :: size_of :: < E > () != 0 { edge_list . entry (& (NoPretty ((e . source () . index () , e . target () . index ())) , e . weight () ,)) ; } else { edge_list . entry (& NoPretty ((e . source () . index () , e . target () . index ()))) ; } } edge_list . finish () } }
    };
}

impl_298!();
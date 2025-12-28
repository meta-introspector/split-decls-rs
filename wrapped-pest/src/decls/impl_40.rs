macro_rules! deps {
    () => {
        RuleType!();
        Pair!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Debug for Pair < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let pair = & mut f . debug_struct ("Pair") ; pair . field ("rule" , & self . as_rule ()) ; if let Some (s) = self . as_node_tag () { pair . field ("node_tag" , & s) ; } pair . field ("span" , & self . as_span ()) . field ("inner" , & self . clone () . into_inner () . collect :: < Vec < _ > > ()) . finish () } }
    };
}

impl_40!();
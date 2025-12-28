macro_rules! deps {
    () => {
        Pair!();
        RuleType!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Display for Pair < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let rule = self . as_rule () ; let start = self . pos (self . start) ; let end = self . pos (self . pair ()) ; let mut pairs = self . clone () . into_inner () . peekable () ; if pairs . peek () . is_none () { write ! (f , "{:?}({}, {})" , rule , start , end) } else { write ! (f , "{:?}({}, {}, [{}])" , rule , start , end , pairs . map (| pair | format ! ("{}" , pair)) . collect ::< Vec < _ >> () . join (", ")) } } }
    };
}

impl_41!();
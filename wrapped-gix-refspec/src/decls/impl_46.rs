macro_rules! deps {
    () => {
        Match!();
        Item!();
        Needle!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Match { fn is_match (& self) -> bool { ! matches ! (self , Match :: None) } fn into_match_outcome < 'a > (self , destination : Needle < 'a > , item : Item < '_ >) -> (bool , Option < Cow < 'a , BStr > >) { let arg = match self { Match :: None => return (false , None) , Match :: Normal => None , Match :: GlobRange (range) => Some ((range , item)) , } ; (true , destination . to_bstr_replace (arg) . into ()) } }
    };
}

impl_46!();
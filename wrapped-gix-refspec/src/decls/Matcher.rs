macro_rules! deps {
    () => {
        Needle!();
    };
}

macro_rules! Matcher {
    () => {
        deps!();
        # [doc = " A type keeping enough information about a ref-spec to be able to efficiently match it against multiple matcher items."] # [derive (Debug)] pub struct Matcher < 'a > { pub (crate) lhs : Option < Needle < 'a > > , pub (crate) rhs : Option < Needle < 'a > > , }
    };
}

Matcher!();
macro_rules! deps {
    () => {
        Parents!();
        QueryPathNode!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < 'a > Parents < 'a > { # [doc = " Get the current query path node, which the next call to `next` will get"] # [doc = " the parents of."] # [must_use] pub fn current (& self) -> & 'a QueryPathNode < 'a > { self . 0 } }
    };
}

impl_342!();
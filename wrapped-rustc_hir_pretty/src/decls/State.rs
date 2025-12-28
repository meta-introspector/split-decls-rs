macro_rules! deps {
    () => {
        PpAnn!();
    };
}

macro_rules! State {
    () => {
        deps!();
        pub struct State < 'a > { pub s : pp :: Printer , comments : Option < Comments < 'a > > , attrs : & 'a dyn Fn (HirId) -> & 'a [hir :: Attribute] , ann : & 'a (dyn PpAnn + 'a) , }
    };
}

State!();
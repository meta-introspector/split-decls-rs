macro_rules! deps {
    () => {
        Comments!();
        PpAnn!();
        Printer!();
    };
}

macro_rules! State {
    () => {
        deps!();
        pub struct State < 'a > { pub s : pp :: Printer , comments : Option < Comments < 'a > > , ann : & 'a (dyn PpAnn + 'a) , is_sdylib_interface : bool , }
    };
}

State!();
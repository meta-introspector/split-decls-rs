macro_rules! deps {
    () => {
        L4Bender!();
        Command!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'a > L4Bender < 'a > { fn new (cmd : Command , sess : & 'a Session) -> L4Bender < 'a > { L4Bender { cmd , sess , hinted_static : false } } fn hint_static (& mut self) { if ! self . hinted_static { self . link_or_cc_arg ("-static") ; self . hinted_static = true ; } } }
    };
}

impl_127!()
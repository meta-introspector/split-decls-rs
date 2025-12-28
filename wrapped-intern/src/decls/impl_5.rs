macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Interned < T > { # [inline] pub fn new_generic < U > (obj : U) -> Self where U : Borrow < T > , Arc < T > : From < U > , { let storage = T :: storage () . get () ; let new_arc = Arc :: from (obj) ; let entry = storage . entry (new_arc . clone ()) . or_insert (()) ; Self { arc : entry . key () . clone () } } }
    };
}

impl_5!()
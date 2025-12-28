macro_rules! deps {
    () => {
        Object!();
        Ref!();
        Store!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < T : Object > Ref < T > { # [doc = " Get a reference to the object associated with this reference from the store"] pub (super) fn get (self , store : & Store < T :: Entry >) -> & T { T :: get_ref (& store . entries [self . index]) . expect ("[loom internal bug] unexpected object stored at reference") } # [doc = " Get a mutable reference to the object associated with this reference"] # [doc = " from the store"] pub (super) fn get_mut (self , store : & mut Store < T :: Entry >) -> & mut T { T :: get_mut (& mut store . entries [self . index]) . expect ("[loom internal bug] unexpected object stored at reference") } }
    };
}

impl_89!()
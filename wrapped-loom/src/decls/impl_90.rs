macro_rules! deps {
    () => {
        Store!();
        Object!();
        Ref!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Ref { # [doc = " Convert a store index `usize` into a ref"] pub (super) fn from_usize (index : usize) -> Ref { Ref { index , _p : PhantomData , } } pub (super) fn downcast < T > (self , store : & Store < T :: Entry >) -> Option < Ref < T > > where T : Object , { T :: get_ref (& store . entries [self . index]) . map (| _ | Ref { index : self . index , _p : PhantomData , }) } }
    };
}

impl_90!()
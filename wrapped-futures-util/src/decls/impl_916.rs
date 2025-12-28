macro_rules! deps {
    () => {
        Close!();
        Sink!();
    };
}

macro_rules! impl_916 {
    () => {
        deps!();
        # [doc = " A future that completes when the sink has finished closing."] # [doc = ""] # [doc = " The sink itself is returned after closing is complete."] impl < 'a , Si : Sink < Item > + Unpin + ? Sized , Item > Close < 'a , Si , Item > { pub (super) fn new (sink : & 'a mut Si) -> Self { Self { sink , _phantom : PhantomData } } }
    };
}

impl_916!()
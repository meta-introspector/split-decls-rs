macro_rules! deps {
    () => {
        Sink!();
        Send!();
        Feed!();
    };
}

macro_rules! impl_954 {
    () => {
        deps!();
        impl < 'a , Si : Sink < Item > + Unpin + ? Sized , Item > Send < 'a , Si , Item > { pub (super) fn new (sink : & 'a mut Si , item : Item) -> Self { Self { feed : Feed :: new (sink , item) } } }
    };
}

impl_954!()
macro_rules! deps {
    () => {
        Feed!();
        Sink!();
    };
}

macro_rules! impl_932 {
    () => {
        deps!();
        impl < 'a , Si : Sink < Item > + Unpin + ? Sized , Item > Feed < 'a , Si , Item > { pub (super) fn new (sink : & 'a mut Si , item : Item) -> Self { Feed { sink , item : Some (item) } } pub (super) fn sink_pin_mut (& mut self) -> Pin < & mut Si > { Pin :: new (self . sink) } pub (super) fn is_item_pending (& self) -> bool { self . item . is_some () } }
    };
}

impl_932!();
macro_rules! deps {
    () => {
        Ready!();
        Sink!();
    };
}

macro_rules! impl_983 {
    () => {
        deps!();
        impl < Si : Sink < Item > , Item > Buffer < Si , Item > { pub (super) fn new (sink : Si , capacity : usize) -> Self { Self { sink , buf : VecDeque :: with_capacity (capacity) , capacity } } delegate_access_inner ! (sink , Si , ()) ; fn try_empty_buffer (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Si :: Error > > { let mut this = self . project () ; ready ! (this . sink . as_mut () . poll_ready (cx)) ? ; while let Some (item) = this . buf . pop_front () { this . sink . as_mut () . start_send (item) ? ; if ! this . buf . is_empty () { ready ! (this . sink . as_mut () . poll_ready (cx)) ? ; } } Poll :: Ready (Ok (())) } }
    };
}

impl_983!();
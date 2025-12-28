macro_rules! deps {
    () => {
        UnboundedReceiver!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < T > Drop for UnboundedReceiver < T > { fn drop (& mut self) { self . close () ; if self . inner . is_some () { loop { match self . next_message () { Poll :: Ready (Some (_)) => { } Poll :: Ready (None) => break , Poll :: Pending => { let state = decode_state (self . inner . as_ref () . unwrap () . state . load (SeqCst)) ; if state . is_closed () { break ; } thread :: yield_now () ; } } } } } }
    };
}

impl_90!()
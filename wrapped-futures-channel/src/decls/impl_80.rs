macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < T > Stream for Receiver < T > { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { match self . next_message () { Poll :: Ready (msg) => { if msg . is_none () { self . inner = None ; } Poll :: Ready (msg) } Poll :: Pending => { self . inner . as_ref () . unwrap () . recv_task . register (cx . waker ()) ; self . next_message () } } } fn size_hint (& self) -> (usize , Option < usize >) { if let Some (inner) = & self . inner { decode_state (inner . state . load (SeqCst)) . size_hint () } else { (0 , Some (0)) } } }
    };
}

impl_80!();
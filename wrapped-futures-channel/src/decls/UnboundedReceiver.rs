macro_rules! deps {
    () => {
        UnboundedInner!();
    };
}

macro_rules! UnboundedReceiver {
    () => {
        deps!();
        # [doc = " The receiving end of an unbounded mpsc channel."] # [doc = ""] # [doc = " This value is created by the [`unbounded`] function."] pub struct UnboundedReceiver < T > { inner : Option < Arc < UnboundedInner < T > > > , }
    };
}

UnboundedReceiver!()
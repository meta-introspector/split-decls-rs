macro_rules! deps {
    () => {
        UnboundedSenderInner!();
    };
}

macro_rules! UnboundedSender {
    () => {
        deps!();
        # [doc = " The transmission end of an unbounded mpsc channel."] # [doc = ""] # [doc = " This value is created by the [`unbounded`] function."] pub struct UnboundedSender < T > (Option < UnboundedSenderInner < T > >) ;
    };
}

UnboundedSender!()
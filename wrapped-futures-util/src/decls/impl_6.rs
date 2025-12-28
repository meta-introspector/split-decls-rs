macro_rules! deps {
    () => {
        Ready!();
        PollOnce!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < F : Future + Unpin > Future for PollOnce < F > { type Output = Poll < F :: Output > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (self . future . poll_unpin (cx)) } }
    };
}

impl_6!()
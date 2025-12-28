macro_rules! deps {
    () => {
        WaitUntil!();
        State!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < F : Future , D : Future > Future for WaitUntil < F , D > { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state { State :: Started => { ready ! (this . deadline . as_mut () . poll (cx)) ; * this . state = State :: PollFuture ; } State :: PollFuture => { let value = ready ! (this . future . as_mut () . poll (cx)) ; * this . state = State :: Completed ; return Poll :: Ready (value) ; } State :: Completed => panic ! ("future polled after completing") , } } } }
    };
}

impl_385!()
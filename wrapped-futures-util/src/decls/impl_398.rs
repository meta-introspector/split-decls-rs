macro_rules! deps {
    () => {
        Ready!();
        Pending!();
        SelectNextSome!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl < St : ? Sized + FusedStream + Unpin > Future for SelectNextSome < '_ , St > { type Output = St :: Item ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { assert ! (! self . stream . is_terminated () , "SelectNextSome polled after terminated") ; if let Some (item) = ready ! (self . stream . poll_next_unpin (cx)) { Poll :: Ready (item) } else { debug_assert ! (self . stream . is_terminated ()) ; cx . waker () . wake_by_ref () ; Poll :: Pending } } }
    };
}

impl_398!()
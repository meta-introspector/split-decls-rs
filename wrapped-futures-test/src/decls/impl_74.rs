macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < St : Stream > Stream for AssertUnmoved < St > { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . poll_with (| s | s . poll_next (cx)) } }
    };
}

impl_74!()
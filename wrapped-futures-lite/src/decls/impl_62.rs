macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T , F > Stream for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < Option < T > > , { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { (& mut self . f) (cx) } }
    };
}

impl_62!()
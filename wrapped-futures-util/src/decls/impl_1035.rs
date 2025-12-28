macro_rules! deps {
    () => {
        Pending!();
        Compat!();
        Ready!();
    };
}

macro_rules! impl_1035 {
    () => {
        deps!();
        impl < St > Stream01 for Compat < St > where St : TryStream03 + Unpin , { type Item = St :: Ok ; type Error = St :: Error ; fn poll (& mut self) -> Poll01 < Option < Self :: Item > , Self :: Error > { with_context (self , | inner , cx | match inner . try_poll_next (cx) ? { task03 :: Poll :: Ready (None) => Ok (Async01 :: Ready (None)) , task03 :: Poll :: Ready (Some (t)) => Ok (Async01 :: Ready (Some (t))) , task03 :: Poll :: Pending => Ok (Async01 :: NotReady) , }) } }
    };
}

impl_1035!()
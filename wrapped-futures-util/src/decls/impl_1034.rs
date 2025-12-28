macro_rules! deps {
    () => {
        Compat!();
    };
}

macro_rules! impl_1034 {
    () => {
        deps!();
        impl < Fut > Future01 for Compat < Fut > where Fut : TryFuture03 + Unpin , { type Item = Fut :: Ok ; type Error = Fut :: Error ; fn poll (& mut self) -> Poll01 < Self :: Item , Self :: Error > { with_context (self , | inner , cx | poll_03_to_01 (inner . try_poll (cx))) } }
    };
}

impl_1034!();
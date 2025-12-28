macro_rules! deps {
    () => {
        Ready!();
        RepeatWith!();
    };
}

macro_rules! impl_755 {
    () => {
        deps!();
        impl < A , F : FnMut () -> A > Stream for RepeatWith < F > { type Item = A ; fn poll_next (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (Some ((& mut self . repeater) ())) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
    };
}

impl_755!()
macro_rules! deps {
    () => {
        PendingOnce!();
        Pending!();
        Ready!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Future for PendingOnce { type Output = () ; fn poll (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Self :: Output > { if self . is_ready { Poll :: Ready (()) } else { self . is_ready = true ; Poll :: Pending } } }
    };
}

impl_11!()
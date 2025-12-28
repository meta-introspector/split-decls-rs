macro_rules! deps {
    () => {
        Pending!();
        TaskFuture!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl Future for TaskFuture { type Output = Box < hyper_task > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match Pin :: new (& mut self . task . as_mut () . unwrap () . future) . poll (cx) { Poll :: Ready (val) => { let mut task = self . task . take () . unwrap () ; task . output = Some (val) ; Poll :: Ready (task) } Poll :: Pending => Poll :: Pending , } } }
    };
}

impl_354!();
macro_rules! deps {
    () => {
        FutureGroup!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < F : Future > Stream for FutureGroup < F > { type Item = < F as Future > :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . poll_next_inner (cx) { Poll :: Ready (Some ((_key , item))) => Poll :: Ready (Some (item)) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => Poll :: Pending , } } }
    };
}

impl_204!();
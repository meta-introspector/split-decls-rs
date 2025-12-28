macro_rules! deps {
    () => {
        Ready!();
        Pending!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < Fut1 : Future , Fut2 : Future > Future for Join < Fut1 , Fut2 > { type Output = (Fut1 :: Output , Fut2 :: Output) ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut all_done = true ; let mut futures = self . project () ; all_done &= futures . fut1 . as_mut () . poll (cx) . is_ready () ; all_done &= futures . fut2 . as_mut () . poll (cx) . is_ready () ; if all_done { Poll :: Ready ((futures . fut1 . take_output () . unwrap () , futures . fut2 . take_output () . unwrap ())) } else { Poll :: Pending } } }
    };
}

impl_212!()
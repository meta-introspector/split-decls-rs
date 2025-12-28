macro_rules! deps {
    () => {
        Pending!();
        Ready!();
        Compat01As03Sink!();
    };
}

macro_rules! impl_1020 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , SinkItem > Stream03 for Compat01As03Sink < S , SinkItem > where S : Stream01 , { type Item = Result < S :: Item , S :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> task03 :: Poll < Option < Self :: Item > > { match self . in_notify (cx , Stream01 :: poll) ? { Async01 :: Ready (Some (t)) => task03 :: Poll :: Ready (Some (Ok (t))) , Async01 :: Ready (None) => task03 :: Poll :: Ready (None) , Async01 :: NotReady => task03 :: Poll :: Pending , } } }
    };
}

impl_1020!()
macro_rules! deps {
    () => {
        Pending!();
        Compat01As03Sink!();
        Ready!();
    };
}

macro_rules! impl_1021 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , SinkItem > Sink03 < SinkItem > for Compat01As03Sink < S , SinkItem > where S : Sink01 < SinkItem = SinkItem > , { type Error = S :: SinkError ; fn start_send (mut self : Pin < & mut Self > , item : SinkItem) -> Result < () , Self :: Error > { debug_assert ! (self . buffer . is_none ()) ; self . buffer = Some (item) ; Ok (()) } fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> task03 :: Poll < Result < () , Self :: Error > > { match self . buffer . take () { Some (item) => match self . in_notify (cx , | f | f . start_send (item)) ? { AsyncSink01 :: Ready => task03 :: Poll :: Ready (Ok (())) , AsyncSink01 :: NotReady (i) => { self . buffer = Some (i) ; task03 :: Poll :: Pending } } , None => task03 :: Poll :: Ready (Ok (())) , } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> task03 :: Poll < Result < () , Self :: Error > > { let item = self . buffer . take () ; match self . in_notify (cx , | f | match item { Some (i) => match f . start_send (i) ? { AsyncSink01 :: Ready => f . poll_complete () . map (| i | (i , None)) , AsyncSink01 :: NotReady (t) => Ok ((Async01 :: NotReady , Some (t))) , } , None => f . poll_complete () . map (| i | (i , None)) , }) ? { (Async01 :: Ready (_) , _) => task03 :: Poll :: Ready (Ok (())) , (Async01 :: NotReady , item) => { self . buffer = item ; task03 :: Poll :: Pending } } } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> task03 :: Poll < Result < () , Self :: Error > > { let item = self . buffer . take () ; let close_started = self . close_started ; let result = self . in_notify (cx , | f | { if ! close_started { if let Some (item) = item { if let AsyncSink01 :: NotReady (item) = f . start_send (item) ? { return Ok ((Async01 :: NotReady , Some (item) , false)) ; } } if let Async01 :: NotReady = f . poll_complete () ? { return Ok ((Async01 :: NotReady , None , false)) ; } } Ok ((< S as Sink01 > :: close (f) ? , None , true)) }) ; match result ? { (Async01 :: Ready (_) , _ , _) => task03 :: Poll :: Ready (Ok (())) , (Async01 :: NotReady , item , close_started) => { self . buffer = item ; self . close_started = close_started ; task03 :: Poll :: Pending } } } }
    };
}

impl_1021!();
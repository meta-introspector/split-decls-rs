macro_rules! deps {
    () => {
        Executor!();
        Unblock!();
        State!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : Write + Send + 'static > AsyncWrite for Unblock < T > { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { loop { match & mut self . state { State :: WithMut (..) | State :: Writing (None , _) | State :: Streaming (..) | State :: Reading (..) | State :: Seeking (..) => { ready ! (self . poll_stop (cx)) ? ; } State :: Idle (io) => { let mut io = io . take () . expect ("inner value was taken out") ; let (mut reader , writer) = pipe (self . cap . unwrap_or (8 * 1024 * 1024)) ; let task = Executor :: spawn (async move { loop { match future :: poll_fn (| cx | reader . poll_drain (cx , & mut io)) . await { Ok (0) => return (io . flush () , io) , Ok (_) => { } Err (err) => { io . flush () . ok () ; return (Err (err) , io) ; } } } }) ; self . state = State :: Writing (Some (writer) , task) ; } State :: Writing (Some (writer) , _) => return writer . poll_fill (cx , buf) , } } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { loop { match & mut self . state { State :: WithMut (..) | State :: Streaming (..) | State :: Writing (..) | State :: Reading (..) | State :: Seeking (..) => { ready ! (self . poll_stop (cx)) ? ; } State :: Idle (_) => return Poll :: Ready (Ok (())) , } } } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { ready ! (Pin :: new (& mut self) . poll_flush (cx)) ? ; self . state = State :: Idle (None) ; Poll :: Ready (Ok (())) } }
    };
}

impl_15!()
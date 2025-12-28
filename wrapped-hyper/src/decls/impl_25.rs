macro_rules! deps {
    () => {
        DecodedLength!();
        Sender!();
        Incoming!();
        UserBody!();
        Kind!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Incoming { # [doc = " Create a `Body` stream with an associated sender half."] # [doc = ""] # [doc = " Useful when wanting to stream chunks from another thread."] # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] # [inline] # [cfg (test)] pub (crate) fn channel () -> (Sender , Incoming) { Self :: new_channel (DecodedLength :: CHUNKED , false) } # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] pub (crate) fn new_channel (content_length : DecodedLength , wanter : bool) -> (Sender , Incoming) { let (data_tx , data_rx) = mpsc :: channel (0) ; let (trailers_tx , trailers_rx) = oneshot :: channel () ; let want = if wanter { WANT_PENDING } else { WANT_READY } ; let (want_tx , want_rx) = watch :: channel (want) ; let tx = Sender { want_rx , data_tx , trailers_tx : Some (trailers_tx) , } ; let rx = Incoming :: new (Kind :: Chan { content_length , want_tx , data_rx , trailers_rx , }) ; (tx , rx) } fn new (kind : Kind) -> Incoming { Incoming { kind } } # [allow (dead_code)] pub (crate) fn empty () -> Incoming { Incoming :: new (Kind :: Empty) } # [cfg (feature = "ffi")] pub (crate) fn ffi () -> Incoming { Incoming :: new (Kind :: Ffi (crate :: ffi :: UserBody :: new ())) } # [cfg (all (feature = "http2" , any (feature = "client" , feature = "server")))] pub (crate) fn h2 (recv : h2 :: RecvStream , mut content_length : DecodedLength , ping : ping :: Recorder ,) -> Self { if ! content_length . is_exact () && recv . is_end_stream () { content_length = DecodedLength :: ZERO ; } Incoming :: new (Kind :: H2 { data_done : false , ping , content_length , recv , }) } # [cfg (feature = "ffi")] pub (crate) fn as_ffi_mut (& mut self) -> & mut crate :: ffi :: UserBody { match self . kind { Kind :: Ffi (ref mut body) => return body , _ => { self . kind = Kind :: Ffi (crate :: ffi :: UserBody :: new ()) ; } } match self . kind { Kind :: Ffi (ref mut body) => body , _ => unreachable ! () , } } }
    };
}

impl_25!()
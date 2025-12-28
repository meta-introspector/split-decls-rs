macro_rules! deps {
    () => {
        Error!();
        Pending!();
        Sender!();
        Result!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] impl Sender { # [doc = " Check to see if this `Sender` can send more data."] pub (crate) fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < crate :: Result < () > > { ready ! (self . poll_want (cx) ?) ; self . data_tx . poll_ready (cx) . map_err (| _ | crate :: Error :: new_closed ()) } fn poll_want (& mut self , cx : & mut Context < '_ >) -> Poll < crate :: Result < () > > { match self . want_rx . load (cx) { WANT_READY => Poll :: Ready (Ok (())) , WANT_PENDING => Poll :: Pending , watch :: CLOSED => Poll :: Ready (Err (crate :: Error :: new_closed ())) , unexpected => unreachable ! ("want_rx value: {}" , unexpected) , } } # [cfg (test)] async fn ready (& mut self) -> crate :: Result < () > { futures_util :: future :: poll_fn (| cx | self . poll_ready (cx)) . await } # [doc = " Send data on data channel when it is ready."] # [cfg (test)] # [allow (unused)] pub (crate) async fn send_data (& mut self , chunk : Bytes) -> crate :: Result < () > { self . ready () . await ? ; self . data_tx . try_send (Ok (chunk)) . map_err (| _ | crate :: Error :: new_closed ()) } # [doc = " Send trailers on trailers channel."] # [allow (unused)] pub (crate) async fn send_trailers (& mut self , trailers : HeaderMap) -> crate :: Result < () > { let tx = match self . trailers_tx . take () { Some (tx) => tx , None => return Err (crate :: Error :: new_closed ()) , } ; tx . send (trailers) . map_err (| _ | crate :: Error :: new_closed ()) } # [doc = " Try to send data on this channel."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `Err(Bytes)` if the channel could not (currently) accept"] # [doc = " another `Bytes`."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This is mostly useful for when trying to send from some other thread"] # [doc = " that doesn't have an async context. If in an async context, prefer"] # [doc = " `send_data()` instead."] # [cfg (feature = "http1")] pub (crate) fn try_send_data (& mut self , chunk : Bytes) -> Result < () , Bytes > { self . data_tx . try_send (Ok (chunk)) . map_err (| err | err . into_inner () . expect ("just sent Ok")) } # [cfg (feature = "http1")] pub (crate) fn try_send_trailers (& mut self , trailers : HeaderMap ,) -> Result < () , Option < HeaderMap > > { let tx = match self . trailers_tx . take () { Some (tx) => tx , None => return Err (None) , } ; tx . send (trailers) . map_err (Some) } # [cfg (test)] pub (crate) fn abort (mut self) { self . send_error (crate :: Error :: new_body_write_aborted ()) ; } pub (crate) fn send_error (& mut self , err : crate :: Error) { let _ = self . data_tx . clone () . try_send (Err (err)) ; } }
    };
}

impl_28!();
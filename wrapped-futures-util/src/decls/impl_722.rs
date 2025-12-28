macro_rules! deps {
    () => {
        ReadState!();
        Ready!();
    };
}

macro_rules! impl_722 {
    () => {
        deps!();
        impl < St > AsyncRead for IntoAsyncRead < St > where St : TryStream < Error = Error > , St :: Ok : AsRef < [u8] > , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { let mut this = self . project () ; loop { match this . state { ReadState :: Ready { chunk , chunk_start } => { let chunk = chunk . as_ref () ; let len = cmp :: min (buf . len () , chunk . len () - * chunk_start) ; buf [.. len] . copy_from_slice (& chunk [* chunk_start .. * chunk_start + len]) ; * chunk_start += len ; if chunk . len () == * chunk_start { * this . state = ReadState :: PendingChunk ; } return Poll :: Ready (Ok (len)) ; } ReadState :: PendingChunk => match ready ! (this . stream . as_mut () . try_poll_next (cx)) { Some (Ok (chunk)) => { if ! chunk . as_ref () . is_empty () { * this . state = ReadState :: Ready { chunk , chunk_start : 0 } ; } } Some (Err (err)) => { * this . state = ReadState :: Eof ; return Poll :: Ready (Err (err)) ; } None => { * this . state = ReadState :: Eof ; return Poll :: Ready (Ok (0)) ; } } , ReadState :: Eof => { return Poll :: Ready (Ok (0)) ; } } } } }
    };
}

impl_722!()
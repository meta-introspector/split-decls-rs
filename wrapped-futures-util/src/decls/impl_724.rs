macro_rules! deps {
    () => {
        ReadState!();
        Ready!();
    };
}

macro_rules! impl_724 {
    () => {
        deps!();
        impl < St > AsyncBufRead for IntoAsyncRead < St > where St : TryStream < Error = Error > , St :: Ok : AsRef < [u8] > , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < & [u8] > > { let mut this = self . project () ; while let ReadState :: PendingChunk = this . state { match ready ! (this . stream . as_mut () . try_poll_next (cx)) { Some (Ok (chunk)) => { if ! chunk . as_ref () . is_empty () { * this . state = ReadState :: Ready { chunk , chunk_start : 0 } ; } } Some (Err (err)) => { * this . state = ReadState :: Eof ; return Poll :: Ready (Err (err)) ; } None => { * this . state = ReadState :: Eof ; return Poll :: Ready (Ok (& [])) ; } } } if let & mut ReadState :: Ready { ref chunk , chunk_start } = this . state { let chunk = chunk . as_ref () ; return Poll :: Ready (Ok (& chunk [chunk_start ..])) ; } Poll :: Ready (Ok (& [])) } fn consume (self : Pin < & mut Self > , amount : usize) { let this = self . project () ; if amount == 0 { return ; } if let ReadState :: Ready { chunk , chunk_start } = this . state { * chunk_start += amount ; debug_assert ! (* chunk_start <= chunk . as_ref () . len ()) ; if * chunk_start >= chunk . as_ref () . len () { * this . state = ReadState :: PendingChunk ; } } else { debug_assert ! (false , "Attempted to consume from IntoAsyncRead without chunk") ; } } }
    };
}

impl_724!()
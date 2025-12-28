macro_rules! deps {
    () => {
        TryChunksError!();
        TryChunksStreamError!();
        Ready!();
    };
}

macro_rules! impl_671 {
    () => {
        deps!();
        impl < St : TryStream > Stream for TryChunks < St > { type Item = Result < Vec < St :: Ok > , TryChunksStreamError < St > > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . try_poll_next (cx)) { Some (item) => match item { Ok (item) => { this . items . push (item) ; if this . items . len () >= * this . cap { return Poll :: Ready (Some (Ok (self . take ()))) ; } } Err (e) => { return Poll :: Ready (Some (Err (TryChunksError (self . take () , e)))) ; } } , None => { let last = if this . items . is_empty () { None } else { let full_buf = mem :: take (this . items) ; Some (full_buf) } ; return Poll :: Ready (last . map (Ok)) ; } } } } fn size_hint (& self) -> (usize , Option < usize >) { let chunk_len = usize :: from (! self . items . is_empty ()) ; let (lower , upper) = self . stream . size_hint () ; let lower = (lower / self . cap) . saturating_add (chunk_len) ; let upper = match upper { Some (x) => x . checked_add (chunk_len) , None => None , } ; (lower , upper) } }
    };
}

impl_671!();
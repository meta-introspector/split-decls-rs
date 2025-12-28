macro_rules! deps {
    () => {
        Pending!();
        TryReadyChunksStreamError!();
        TryReadyChunksError!();
        Ready!();
    };
}

macro_rules! impl_682 {
    () => {
        deps!();
        impl < St : TryStream > Stream for TryReadyChunks < St > { type Item = Result < Vec < St :: Ok > , TryReadyChunksStreamError < St > > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . as_mut () . project () ; let mut items : Vec < St :: Ok > = Vec :: new () ; loop { match this . stream . as_mut () . poll_next (cx) { Poll :: Pending => { return if items . is_empty () { Poll :: Pending } else { Poll :: Ready (Some (Ok (items))) } } Poll :: Ready (Some (Ok (item))) => { if items . is_empty () { items . reserve_exact (* this . cap) ; } items . push (item) ; if items . len () >= * this . cap { return Poll :: Ready (Some (Ok (items))) ; } } Poll :: Ready (Some (Err (e))) => { return Poll :: Ready (Some (Err (TryReadyChunksError (items , e)))) ; } Poll :: Ready (None) => { let last = if items . is_empty () { None } else { Some (Ok (items)) } ; return Poll :: Ready (last) ; } } } } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . stream . size_hint () ; let lower = lower / self . cap ; (lower , upper) } }
    };
}

impl_682!()
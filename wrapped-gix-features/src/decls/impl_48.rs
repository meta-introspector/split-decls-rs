macro_rules! deps {
    () => {
        EagerIter!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < I > EagerIter < I > where I : Iterator + Send + 'static , < I as Iterator > :: Item : Send , { # [doc = " Return a new `EagerIter` which evaluates `iter` in its own thread,"] # [doc = " with a given `chunk_size` allowing a maximum `chunks_in_flight`."] # [doc = ""] # [doc = " * `chunk_size` describes how many items returned by `iter` will be a single item of this `EagerIter`."] # [doc = "   This helps to reduce the overhead imposed by transferring many small items."] # [doc = "   If this number is 1, each item will become a single chunk. 0 is invalid."] # [doc = " * `chunks_in_flight` describes how many chunks can be kept in memory in case the consumer of the `EagerIter`s items"] # [doc = "   isn't consuming them fast enough. Setting this number to 0 effectively turns off any caching, but blocks `EagerIter`"] # [doc = "   if its items aren't consumed fast enough."] pub fn new (iter : I , chunk_size : usize , chunks_in_flight : usize) -> Self { let (sender , receiver) = std :: sync :: mpsc :: sync_channel (chunks_in_flight) ; let size_hint = iter . size_hint () ; assert ! (chunk_size > 0 , "non-zero chunk size is needed") ; std :: thread :: spawn (move | | { let mut out = Vec :: with_capacity (chunk_size) ; for item in iter { out . push (item) ; if out . len () == chunk_size { if sender . send (out) . is_err () { return ; } out = Vec :: with_capacity (chunk_size) ; } } if ! out . is_empty () { sender . send (out) . ok () ; } }) ; EagerIter { receiver , chunk : None , size_hint , } } fn fill_buf_and_pop (& mut self) -> Option < I :: Item > { self . chunk = self . receiver . recv () . ok () . map (| v | { assert ! (! v . is_empty ()) ; v . into_iter () }) ; self . chunk . as_mut () . and_then (Iterator :: next) } }
    };
}

impl_48!()
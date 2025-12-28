macro_rules! deps {
    () => {
        HeaderCaseMap!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] impl HeaderCaseMap { # [doc = " Returns a view of all spellings associated with that header name,"] # [doc = " in the order they were found."] # [cfg (feature = "client")] pub (crate) fn get_all < 'a > (& 'a self , name : & HeaderName ,) -> impl Iterator < Item = impl AsRef < [u8] > + 'a > + 'a { self . get_all_internal (name) } # [doc = " Returns a view of all spellings associated with that header name,"] # [doc = " in the order they were found."] # [cfg (any (feature = "client" , feature = "server"))] pub (crate) fn get_all_internal (& self , name : & HeaderName) -> ValueIter < '_ , Bytes > { self . 0 . get_all (name) . into_iter () } # [cfg (any (feature = "client" , feature = "server"))] pub (crate) fn default () -> Self { Self (Default :: default ()) } # [cfg (any (test , feature = "ffi"))] pub (crate) fn insert (& mut self , name : HeaderName , orig : Bytes) { self . 0 . insert (name , orig) ; } # [cfg (any (feature = "client" , feature = "server"))] pub (crate) fn append < N > (& mut self , name : N , orig : Bytes) where N : IntoHeaderName , { self . 0 . append (name , orig) ; } }
    };
}

impl_155!();
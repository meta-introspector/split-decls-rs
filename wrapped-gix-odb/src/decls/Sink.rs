macro_rules! Sink {
    () => {
        # [doc = ""] # [doc = " It can optionally compress the content, similarly to what would happen when using a [`loose::Store`]."] # [doc = ""] # [derive (Clone)] pub struct Sink { compressor : Option < RefCell < deflate :: Write < std :: io :: Sink > > > , object_hash : gix_hash :: Kind , }
    };
}

Sink!();
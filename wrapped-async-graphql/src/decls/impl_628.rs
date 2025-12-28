macro_rules! deps {
    () => {
        Context!();
        Result!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl < T : AsyncRead > Stream for ReaderStream < T > { type Item = io :: Result < Vec < u8 > > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Option < Self :: Item > > { let this = self . project () ; Poll :: Ready (match futures_util :: ready ! (this . reader . poll_read (cx , this . buf) ?) { 0 => None , size => Some (Ok (this . buf [.. size] . to_vec ())) , } ,) } }
    };
}

impl_628!()
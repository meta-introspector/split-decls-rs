macro_rules! deps {
    () => {
        SchemePermission!();
        Ssh!();
        Http!();
    };
}

macro_rules! impl_985 {
    () => {
        deps!();
        # [doc = " Access"] impl SchemePermission { pub fn allow (& self , scheme : & gix_url :: Scheme) -> bool { self . allow_per_scheme . get (scheme) . or (self . allow . as_ref ()) . map_or_else (| | { use gix_url :: Scheme :: * ; match scheme { File | Git | Ssh | Http | Https => true , Ext (_) => false , } } , | allow | allow . to_bool (self . user_allowed) ,) } }
    };
}

impl_985!()
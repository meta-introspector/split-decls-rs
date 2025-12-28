macro_rules! deps {
    () => {
        Error!();
        Connection!();
        TransportWithoutIO!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < R , W > client :: TransportWithoutIO for Connection < R , W > where R : AsyncRead + Unpin , W : AsyncWrite + Unpin , { fn to_url (& self) -> Cow < '_ , BStr > { self . state . custom_url . as_ref () . map_or_else (| | { let mut possibly_lossy_url = self . state . path . clone () ; possibly_lossy_url . insert_str (0 , "file://") ; Cow :: Owned (possibly_lossy_url) } , | url | Cow :: Borrowed (url . as_ref ()) ,) } fn connection_persists_across_multiple_requests (& self) -> bool { true } fn configure (& mut self , _config : & dyn std :: any :: Any) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { Ok (()) } }
    };
}

impl_168!();
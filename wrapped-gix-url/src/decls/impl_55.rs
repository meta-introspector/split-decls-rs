macro_rules! deps {
    () => {
        Url!();
        Scheme!();
        Error!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [doc = " Serialization"] impl Url { # [doc = " Write this URL losslessly to `out`, ready to be parsed again."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { if self . serialize_alternative_form && (self . scheme == Scheme :: File || self . scheme == Scheme :: Ssh) && self . password . is_none () && self . port . is_none () { self . write_alternative_form_to (out) } else { self . write_canonical_form_to (out) } } fn write_canonical_form_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { out . write_all (self . scheme . as_str () . as_bytes ()) ? ; out . write_all (b"://") ? ; match (& self . user , & self . host) { (Some (user) , Some (host)) => { out . write_all (percent_encode (user) . as_bytes ()) ? ; if let Some (password) = & self . password { out . write_all (b":") ? ; out . write_all (percent_encode (password) . as_bytes ()) ? ; } out . write_all (b"@") ? ; out . write_all (host . as_bytes ()) ? ; } (None , Some (host)) => { out . write_all (host . as_bytes ()) ? ; } (None , None) => { } (Some (_user) , None) => { return Err (std :: io :: Error :: other ("Invalid URL structure: user specified without host" ,)) ; } } if let Some (port) = & self . port { write ! (out , ":{port}") ? ; } out . write_all (& self . path) ? ; Ok (()) } fn write_alternative_form_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { match (& self . user , & self . host) { (Some (user) , Some (host)) => { out . write_all (user . as_bytes ()) ? ; assert ! (self . password . is_none () , "BUG: cannot serialize password in alternative form") ; out . write_all (b"@") ? ; out . write_all (host . as_bytes ()) ? ; } (None , Some (host)) => { out . write_all (host . as_bytes ()) ? ; } (None , None) => { } (Some (_user) , None) => { return Err (std :: io :: Error :: other ("Invalid URL structure: user specified without host" ,)) ; } } assert ! (self . port . is_none () , "BUG: cannot serialize port in alternative form") ; if self . scheme == Scheme :: Ssh { out . write_all (b":") ? ; } out . write_all (& self . path) ? ; Ok (()) } # [doc = " Transform ourselves into a binary string, losslessly, or fail if the URL is malformed due to host or user parts being incorrect."] pub fn to_bstring (& self) -> BString { let mut buf = Vec :: with_capacity ((5 + 3) + self . user . as_ref () . map (String :: len) . unwrap_or_default () + 1 + self . host . as_ref () . map (String :: len) . unwrap_or_default () + self . port . map (| _ | 5) . unwrap_or_default () + self . path . len () ,) ; self . write_to (& mut buf) . expect ("io cannot fail in memory") ; buf . into () } }
    };
}

impl_55!()
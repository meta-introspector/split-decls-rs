macro_rules! deps {
    () => {
        EntriesToBytesIter!();
        Item!();
        Error!();
        Entry!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < I , W > Iterator for EntriesToBytesIter < I , W > where I : Iterator < Item = Result < input :: Entry , input :: Error > > , W : std :: io :: Read + std :: io :: Write + std :: io :: Seek , { # [doc = " The amount of bytes written to `out` if `Ok` or the error `E` received from the input."] type Item = Result < input :: Entry , input :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . is_done { return None ; } match self . input . next () { Some (res) => Some (match res { Ok (entry) => self . next_inner (entry) . and_then (| mut entry | { if self . input . peek () . is_none () { self . write_header_and_digest (Some (& mut entry)) . map (| _ | entry) } else { Ok (entry) } }) . map_err (input :: Error :: from) , Err (err) => { self . is_done = true ; Err (err) } }) , None => match self . write_header_and_digest (None) { Ok (_) => None , Err (err) => Some (Err (err . into ())) , } , } } fn size_hint (& self) -> (usize , Option < usize >) { self . input . size_hint () } }
    };
}

impl_163!()
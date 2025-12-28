macro_rules! deps {
    () => {
        FromEntriesIter!();
        Entry!();
        Item!();
        Error!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < I , W , E > Iterator for FromEntriesIter < I , W > where I : Iterator < Item = Result < Vec < output :: Entry > , E > > , W : std :: io :: Write , E : std :: error :: Error + 'static , { # [doc = " The amount of bytes written to `out` if `Ok` or the error `E` received from the input."] type Item = Result < u64 , Error < E > > ; fn next (& mut self) -> Option < Self :: Item > { if self . is_done { return None ; } Some (match self . next_inner () { Err (err) => { self . is_done = true ; Err (err) } Ok (written) => Ok (written) , }) } }
    };
}

impl_186!()
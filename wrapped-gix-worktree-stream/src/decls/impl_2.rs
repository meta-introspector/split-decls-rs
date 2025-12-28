macro_rules! deps {
    () => {
        Error!();
        Entry!();
        Stream!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Stream { # [doc = " Access the next entry of the stream or `None` if there is nothing more to read."] pub fn next_entry (& mut self) -> Result < Option < Entry < '_ > > , Error > { assert ! (self . path_buf . is_some () , "BUG: must consume and drop entry before getting the next one") ; self . extra_entries . take () ; let res = protocol :: read_entry_info (& mut self . read , self . path_buf . as_mut () . expect ("set while producing an entry") ,) ; match res { Ok ((remaining , mode , id)) => { if let Some (err) = self . err . lock () . take () { return Err (err) ; } Ok (Some (Entry { path_buf : self . path_buf . take () , parent : self , id , mode , remaining , })) } Err (err) => { if let Some (err) = self . err . lock () . take () { return Err (err) ; } if err . kind () == ErrorKind :: UnexpectedEof { return Ok (None) ; } Err (err . into ()) } } } }
    };
}

impl_2!();
macro_rules! deps {
    () => {
        Error!();
        ReadFilterOutput!();
        Driver!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl std :: io :: Read for ReadFilterOutput { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { match self . inner . as_mut () { Some (inner) => { let num_read = match inner . read (buf) { Ok (n) => n , Err (e) => { if let Some (mut write_thread) = self . write_thread . take () { if let Err (_thread_err) = write_thread . join () { gix_trace :: debug ! (thread_err = % _thread_err , read_err = % e , "write to stdin error during failed read") ; } } return Err (e) ; } } ; if num_read == 0 { self . inner . take () ; if let Some (mut write_thread) = self . write_thread . take () { write_thread . join () ? ; } if let Some ((mut child , cmd)) = self . child . take () { let status = child . wait () ? ; if ! status . success () { return Err (std :: io :: Error :: other (format ! ("Driver process {cmd:?} failed"))) ; } } } Ok (num_read) } None => Ok (0) , } } }
    };
}

impl_51!()
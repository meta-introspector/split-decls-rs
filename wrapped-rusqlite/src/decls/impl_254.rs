macro_rules! deps {
    () => {
        Result!();
        ChangesetIter!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl ChangesetIter < '_ > { # [doc = " Create an iterator on `input`"] # [inline] pub fn start_strm < 'input > (input : & & 'input mut dyn Read) -> Result < ChangesetIter < 'input > > { let mut it = ptr :: null_mut () ; check (unsafe { ffi :: sqlite3changeset_start_strm (& mut it as * mut * mut _ , Some (x_input) , input as * const & mut dyn Read as * mut c_void ,) }) ? ; Ok (ChangesetIter { phantom : PhantomData , it , item : None , }) } }
    };
}

impl_254!()
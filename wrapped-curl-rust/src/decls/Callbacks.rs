macro_rules! deps {
    () => {
        InfoType!();
        Error!();
        ReadError!();
        WriteError!();
        SeekResult!();
    };
}

macro_rules! Callbacks {
    () => {
        deps!();
        # [derive (Default)] struct Callbacks < 'a > { write : Option < Box < dyn FnMut (& [u8]) -> Result < usize , WriteError > + 'a > > , read : Option < Box < dyn FnMut (& mut [u8]) -> Result < usize , ReadError > + 'a > > , seek : Option < Box < dyn FnMut (SeekFrom) -> SeekResult + 'a > > , debug : Option < Box < dyn FnMut (InfoType , & [u8]) + 'a > > , header : Option < Box < dyn FnMut (& [u8]) -> bool + 'a > > , progress : Option < Box < dyn FnMut (f64 , f64 , f64 , f64) -> bool + 'a > > , ssl_ctx : Option < Box < dyn FnMut (* mut c_void) -> Result < () , Error > + 'a > > , }
    };
}

Callbacks!();
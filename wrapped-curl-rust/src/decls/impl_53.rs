macro_rules! deps {
    () => {
        SeekResult!();
        InfoType!();
        Callbacks!();
        ReadError!();
        Error!();
        Transfer!();
        WriteError!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'easy , 'data > Transfer < 'easy , 'data > { # [doc = " Same as `Easy::write_function`, just takes a non `'static` lifetime"] # [doc = " corresponding to the lifetime of this transfer."] pub fn write_function < F > (& mut self , f : F) -> Result < () , Error > where F : FnMut (& [u8]) -> Result < usize , WriteError > + 'data , { self . data . write = Some (Box :: new (f)) ; Ok (()) } # [doc = " Same as `Easy::read_function`, just takes a non `'static` lifetime"] # [doc = " corresponding to the lifetime of this transfer."] pub fn read_function < F > (& mut self , f : F) -> Result < () , Error > where F : FnMut (& mut [u8]) -> Result < usize , ReadError > + 'data , { self . data . read = Some (Box :: new (f)) ; Ok (()) } # [doc = " Same as `Easy::seek_function`, just takes a non `'static` lifetime"] # [doc = " corresponding to the lifetime of this transfer."] pub fn seek_function < F > (& mut self , f : F) -> Result < () , Error > where F : FnMut (SeekFrom) -> SeekResult + 'data , { self . data . seek = Some (Box :: new (f)) ; Ok (()) } # [doc = " Same as `Easy::progress_function`, just takes a non `'static` lifetime"] # [doc = " corresponding to the lifetime of this transfer."] pub fn progress_function < F > (& mut self , f : F) -> Result < () , Error > where F : FnMut (f64 , f64 , f64 , f64) -> bool + 'data , { self . data . progress = Some (Box :: new (f)) ; Ok (()) } # [doc = " Same as `Easy::ssl_ctx_function`, just takes a non `'static`"] # [doc = " lifetime corresponding to the lifetime of this transfer."] pub fn ssl_ctx_function < F > (& mut self , f : F) -> Result < () , Error > where F : FnMut (* mut c_void) -> Result < () , Error > + Send + 'data , { self . data . ssl_ctx = Some (Box :: new (f)) ; Ok (()) } # [doc = " Same as `Easy::debug_function`, just takes a non `'static` lifetime"] # [doc = " corresponding to the lifetime of this transfer."] pub fn debug_function < F > (& mut self , f : F) -> Result < () , Error > where F : FnMut (InfoType , & [u8]) + 'data , { self . data . debug = Some (Box :: new (f)) ; Ok (()) } # [doc = " Same as `Easy::header_function`, just takes a non `'static` lifetime"] # [doc = " corresponding to the lifetime of this transfer."] pub fn header_function < F > (& mut self , f : F) -> Result < () , Error > where F : FnMut (& [u8]) -> bool + 'data , { self . data . header = Some (Box :: new (f)) ; Ok (()) } # [doc = " Same as `Easy::perform`."] pub fn perform (& self) -> Result < () , Error > { let inner = self . easy . inner . get_ref () ; inner . borrowed . set (& * self . data as * const _ as * mut _) ; struct Reset < 'a > (& 'a Cell < * mut Callbacks < 'static > >) ; impl < 'a > Drop for Reset < 'a > { fn drop (& mut self) { self . 0 . set (ptr :: null_mut ()) ; } } let _reset = Reset (& inner . borrowed) ; self . easy . do_perform () } # [doc = " Same as `Easy::upkeep`"] # [cfg (feature = "upkeep_7_62_0")] pub fn upkeep (& self) -> Result < () , Error > { self . easy . upkeep () } # [doc = " Same as `Easy::unpause_read`."] pub fn unpause_read (& self) -> Result < () , Error > { self . easy . unpause_read () } # [doc = " Same as `Easy::unpause_write`"] pub fn unpause_write (& self) -> Result < () , Error > { self . easy . unpause_write () } }
    };
}

impl_53!()
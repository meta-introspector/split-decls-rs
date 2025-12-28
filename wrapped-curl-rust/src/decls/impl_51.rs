macro_rules! deps {
    () => {
        Error!();
        WriteError!();
        EasyData!();
        ReadError!();
        Handler!();
        SeekResult!();
        InfoType!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Handler for EasyData { fn write (& mut self , data : & [u8]) -> Result < usize , WriteError > { unsafe { match self . callback (| s | & mut s . write) { Some (write) => write (data) , None => Ok (data . len ()) , } } } fn read (& mut self , data : & mut [u8]) -> Result < usize , ReadError > { unsafe { match self . callback (| s | & mut s . read) { Some (read) => read (data) , None => Ok (0) , } } } fn seek (& mut self , whence : SeekFrom) -> SeekResult { unsafe { match self . callback (| s | & mut s . seek) { Some (seek) => seek (whence) , None => SeekResult :: CantSeek , } } } fn debug (& mut self , kind : InfoType , data : & [u8]) { unsafe { match self . callback (| s | & mut s . debug) { Some (debug) => debug (kind , data) , None => handler :: debug (kind , data) , } } } fn header (& mut self , data : & [u8]) -> bool { unsafe { match self . callback (| s | & mut s . header) { Some (header) => header (data) , None => true , } } } fn progress (& mut self , dltotal : f64 , dlnow : f64 , ultotal : f64 , ulnow : f64) -> bool { unsafe { match self . callback (| s | & mut s . progress) { Some (progress) => progress (dltotal , dlnow , ultotal , ulnow) , None => true , } } } fn ssl_ctx (& mut self , cx : * mut c_void) -> Result < () , Error > { unsafe { match self . callback (| s | & mut s . ssl_ctx) { Some (ssl_ctx) => ssl_ctx (cx) , None => handler :: ssl_ctx (cx) , } } } }
    };
}

impl_51!()
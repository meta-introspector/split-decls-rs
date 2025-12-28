macro_rules! deps {
    () => {
        RawStream!();
        AutoStream!();
        AsLockedWrite!();
        StreamInner!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < S > std :: io :: Write for AutoStream < S > where S : RawStream + AsLockedWrite , { # [inline] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { match & mut self . inner { StreamInner :: PassThrough (w) => w . as_locked_write () . write (buf) , StreamInner :: Strip (w) => w . write (buf) , # [cfg (all (windows , feature = "wincon"))] StreamInner :: Wincon (w) => w . write (buf) , } } # [inline] fn write_vectored (& mut self , bufs : & [std :: io :: IoSlice < '_ >]) -> std :: io :: Result < usize > { match & mut self . inner { StreamInner :: PassThrough (w) => w . as_locked_write () . write_vectored (bufs) , StreamInner :: Strip (w) => w . write_vectored (bufs) , # [cfg (all (windows , feature = "wincon"))] StreamInner :: Wincon (w) => w . write_vectored (bufs) , } } # [inline] fn flush (& mut self) -> std :: io :: Result < () > { match & mut self . inner { StreamInner :: PassThrough (w) => w . as_locked_write () . flush () , StreamInner :: Strip (w) => w . flush () , # [cfg (all (windows , feature = "wincon"))] StreamInner :: Wincon (w) => w . flush () , } } # [inline] fn write_all (& mut self , buf : & [u8]) -> std :: io :: Result < () > { match & mut self . inner { StreamInner :: PassThrough (w) => w . as_locked_write () . write_all (buf) , StreamInner :: Strip (w) => w . write_all (buf) , # [cfg (all (windows , feature = "wincon"))] StreamInner :: Wincon (w) => w . write_all (buf) , } } # [inline] fn write_fmt (& mut self , args : std :: fmt :: Arguments < '_ >) -> std :: io :: Result < () > { match & mut self . inner { StreamInner :: PassThrough (w) => w . as_locked_write () . write_fmt (args) , StreamInner :: Strip (w) => w . write_fmt (args) , # [cfg (all (windows , feature = "wincon"))] StreamInner :: Wincon (w) => w . write_fmt (args) , } } }
    };
}

impl_100!();
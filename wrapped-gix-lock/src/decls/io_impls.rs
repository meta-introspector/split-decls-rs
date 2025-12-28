macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! io_impls {
    () => {
        deps!();
        mod io_impls { use std :: { io , io :: SeekFrom } ; use super :: File ; impl io :: Write for File { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . with_mut (| f | f . write (buf)) ? } fn flush (& mut self) -> io :: Result < () > { self . inner . with_mut (io :: Write :: flush) ? } } impl io :: Seek for File { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . inner . with_mut (| f | f . seek (pos)) ? } } impl io :: Read for File { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . with_mut (| f | f . read (buf)) ? } } }
    };
}

io_impls!()
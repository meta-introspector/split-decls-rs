macro_rules! deps {
    () => {
        Writable!();
        Handle!();
    };
}

macro_rules! io_impls {
    () => {
        deps!();
        mod io_impls { use std :: { io , io :: SeekFrom } ; use super :: { Handle , Writable } ; impl io :: Write for Handle < Writable > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . with_mut (| f | f . write (buf)) ? } fn flush (& mut self) -> io :: Result < () > { self . with_mut (io :: Write :: flush) ? } } impl io :: Seek for Handle < Writable > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . with_mut (| f | f . seek (pos)) ? } } impl io :: Read for Handle < Writable > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . with_mut (| f | f . read (buf)) ? } } }
    };
}

io_impls!();
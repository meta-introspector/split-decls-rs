macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! Write {
    () => {
        deps!();
        # [doc = " `no_std` compatible `std::io::Write trait`"] # [doc = ""] # [doc = " Will get removed once there is a standard way in either `core` or `alloc`."] pub trait Write { # [doc = " Write a buffer into this writer."] fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > ; # [doc = " Flush this output stream."] fn flush (& mut self) -> crate :: Result < () > ; # [doc = " Attempts to write an entire buffer into this writer."] fn write_all (& mut self , mut buf : & [u8]) -> crate :: Result < () > { while ! buf . is_empty () { match self . write (buf) { Ok (0) => { return Err (Error :: WriteZero ("could not write any byte")) ; } Ok (n) => buf = & buf [n ..] , Err (Error :: Interrupted) => { } Err (e) => return Err (e) , } } Ok (()) } }
    };
}

Write!()
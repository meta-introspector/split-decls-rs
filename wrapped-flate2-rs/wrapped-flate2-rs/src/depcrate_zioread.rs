// Generated macro for read (function)
macro_rules! Depcrate_zioread {
() => {
// Module: crate::zio
// Provides: {"read"}
// Dependencies: {}
pub fn read < R , D > (obj : & mut R , data : & mut D , dst : & mut [u8]) -> io :: Result < usize > where R : BufRead , D : Ops , { loop { let (read , consumed , ret , eof) ; { let input = obj . fill_buf () ? ; eof = input . is_empty () ; let before_out = data . total_out () ; let before_in = data . total_in () ; let flush = if eof { D :: Flush :: finish () } else { D :: Flush :: none () } ; ret = data . run (input , dst , flush) ; read = (data . total_out () - before_out) as usize ; consumed = (data . total_in () - before_in) as usize ; } obj . consume (consumed) ; match ret { Ok (Status :: Ok | Status :: BufError) if read == 0 && ! eof && ! dst . is_empty () => continue , Ok (Status :: Ok | Status :: BufError | Status :: StreamEnd) => return Ok (read) , Err (..) => { return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "corrupt deflate stream" ,)) } } } }
};
}

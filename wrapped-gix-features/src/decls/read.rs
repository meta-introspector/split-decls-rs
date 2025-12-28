macro_rules! deps {
    () => {
        FlushDecompress!();
        Read!();
        Decompress!();
        Status!();
    };
}

macro_rules! read {
    () => {
        deps!();
        # [doc = " Read bytes from `rd` and decompress them using `state` into a pre-allocated fitting buffer `dst`, returning the amount of bytes written."] pub fn read (rd : & mut impl BufRead , state : & mut Decompress , mut dst : & mut [u8]) -> io :: Result < usize > { let mut total_written = 0 ; loop { let (written , consumed , ret , eof) ; { let input = rd . fill_buf () ? ; eof = input . is_empty () ; let before_out = state . total_out () ; let before_in = state . total_in () ; let flush = if eof { FlushDecompress :: Finish } else { FlushDecompress :: None } ; ret = state . decompress (input , dst , flush) ; written = (state . total_out () - before_out) as usize ; total_written += written ; dst = & mut dst [written ..] ; consumed = (state . total_in () - before_in) as usize ; } rd . consume (consumed) ; match ret { Ok (Status :: StreamEnd) => return Ok (total_written) , Ok (Status :: Ok | Status :: BufError) if eof || dst . is_empty () => return Ok (total_written) , Ok (Status :: Ok | Status :: BufError) if consumed != 0 || written != 0 => continue , Ok (Status :: Ok | Status :: BufError) => unreachable ! ("Definitely a bug somewhere") , Err (..) => return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "corrupt deflate stream")) , } } }
    };
}

read!()
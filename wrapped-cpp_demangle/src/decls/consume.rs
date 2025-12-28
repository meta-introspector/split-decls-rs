macro_rules! deps {
    () => {
        IndexStr!();
        Error!();
        Result!();
    };
}

macro_rules! consume {
    () => {
        deps!();
        # [doc = " Expect and consume the given byte str, and return the advanced `IndexStr` if"] # [doc = " we saw the expectation. Otherwise return an error of kind"] # [doc = " `error::Error::UnexpectedText` if the input doesn't match, or"] # [doc = " `error::Error::UnexpectedEnd` if it isn't long enough."] # [inline] fn consume < 'a > (expected : & [u8] , input : IndexStr < 'a >) -> Result < IndexStr < 'a > > { match input . try_split_at (expected . len ()) { Some ((head , tail)) if head == expected => Ok (tail) , Some (_) => Err (error :: Error :: UnexpectedText) , None => Err (error :: Error :: UnexpectedEnd) , } }
    };
}

consume!();
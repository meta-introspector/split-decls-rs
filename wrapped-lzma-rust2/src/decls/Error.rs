macro_rules! Error {
    () => {
        # [doc = " `no_std` compatible error type."] # [doc = ""] # [doc = " Will get removed once `std::io::Read` and `std::io::Write` are available for `no_std`."] # [derive (Debug , Copy , Clone)] pub enum Error { # [doc = " End of file reached unexpectedly."] Eof , # [doc = " Operation was interrupted."] Interrupted , # [doc = " Invalid data encountered."] InvalidData (& 'static str) , # [doc = " Invalid input provided."] InvalidInput (& 'static str) , # [doc = " Out of memory error."] OutOfMemory (& 'static str) , # [doc = " Other error."] Other (& 'static str) , # [doc = " Unsupported operation."] Unsupported (& 'static str) , # [doc = " Could not write any bytes."] WriteZero (& 'static str) , }
    };
}

Error!();
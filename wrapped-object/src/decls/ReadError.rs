macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ReadError {
    () => {
        deps!();
        trait ReadError < T > { fn read_error (self , error : & 'static str) -> Result < T > ; }
    };
}

ReadError!();
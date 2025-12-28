macro_rules! deps {
    () => {
        Writer!();
        EncodeError!();
        IoWriter!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < W : std :: io :: Write > Writer for IoWriter < '_ , W > { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { self . writer . write_all (bytes) . map_err (| inner | EncodeError :: Io { inner , index : self . bytes_written , }) ? ; self . bytes_written += bytes . len () ; Ok (()) } }
    };
}

impl_93!();
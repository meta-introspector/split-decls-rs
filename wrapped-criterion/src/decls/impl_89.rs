macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [cfg (feature = "csv_output")] impl From < CsvError > for Error { fn from (other : CsvError) -> Error { Error :: CsvError (other) } }
    };
}

impl_89!()
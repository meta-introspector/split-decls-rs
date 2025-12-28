macro_rules! deps {
    () => {
        Error!();
        FromSqlError!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl Error for FromSqlError { fn source (& self) -> Option < & (dyn Error + 'static) > { if let Self :: Other (ref err) = self { Some (& * * err) } else { None } } }
    };
}

impl_325!()
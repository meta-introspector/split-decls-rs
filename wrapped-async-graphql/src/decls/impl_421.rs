macro_rules! deps {
    () => {
        SchemaError!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl < T : Into < String > > From < T > for SchemaError { fn from (err : T) -> Self { SchemaError (err . into ()) } }
    };
}

impl_421!();
macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! SchemaError {
    () => {
        deps!();
        # [doc = " An error can occur when building dynamic schema"] # [derive (Debug , thiserror :: Error , Eq , PartialEq)] # [error ("{0}")] pub struct SchemaError (pub String) ;
    };
}

SchemaError!();
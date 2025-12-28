macro_rules! deps {
    () => {
        Deserializer!();
        Read!();
        IoRead!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < R > Deserializer < read :: IoRead < R > > where R : crate :: io :: Read , { # [doc = " Creates a JSON deserializer from an `io::Read`."] # [doc = ""] # [doc = " Reader-based deserializers do not support deserializing borrowed types"] # [doc = " like `&str`, since the `std::io::Read` trait has no non-copying methods"] # [doc = " -- everything it does involves copying bytes out of the data source."] pub fn from_reader (reader : R) -> Self { Deserializer :: new (read :: IoRead :: new (reader)) } }
    };
}

impl_14!();
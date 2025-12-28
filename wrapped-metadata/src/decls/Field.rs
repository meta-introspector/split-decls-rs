macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! Field {
    () => {
        deps!();
        pub struct Field { pub Flags : FieldAttributes , pub Name : id :: StringId , pub Signature : id :: BlobId , }
    };
}

Field!();
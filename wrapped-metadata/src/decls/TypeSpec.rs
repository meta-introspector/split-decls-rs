macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! TypeSpec {
    () => {
        deps!();
        pub struct TypeSpec { pub Signature : id :: BlobId , }
    };
}

TypeSpec!()
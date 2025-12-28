macro_rules! FallbackS {
    () => {
        pub struct FallbackS < F > (F) ;
    };
}

FallbackS!()
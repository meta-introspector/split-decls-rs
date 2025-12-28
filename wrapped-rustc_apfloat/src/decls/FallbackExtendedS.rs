macro_rules! FallbackExtendedS {
    () => {
        pub struct FallbackExtendedS < F > (F) ;
    };
}

FallbackExtendedS!()
macro_rules! UncheckedOptionExt {
    () => {
        pub trait UncheckedOptionExt < T > { unsafe fn unchecked_unwrap (self) -> T ; }
    };
}

UncheckedOptionExt!();
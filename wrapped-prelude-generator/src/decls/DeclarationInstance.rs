macro_rules! DeclarationInstance {
    () => {
        pub struct DeclarationInstance < T > { inner : T , }
    };
}

DeclarationInstance!()
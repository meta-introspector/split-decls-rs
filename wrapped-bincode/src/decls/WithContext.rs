macro_rules! WithContext {
    () => {
        pub struct WithContext < 'a , D : ? Sized , C > { pub (crate) decoder : & 'a mut D , pub (crate) context : C , }
    };
}

WithContext!()
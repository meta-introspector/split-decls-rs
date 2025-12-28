macro_rules! Indented {
    () => {
        struct Indented < 'a , D > { inner : & 'a mut D , number : Option < usize > , started : bool , }
    };
}

Indented!()
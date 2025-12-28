macro_rules! deps {
    () => {
        InlayHintLabelPart!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl std :: hash :: Hash for InlayHintLabelPart { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . text . hash (state) ; self . linked_location . is_some () . hash (state) ; self . tooltip . is_some () . hash (state) ; } }
    };
}

impl_276!()
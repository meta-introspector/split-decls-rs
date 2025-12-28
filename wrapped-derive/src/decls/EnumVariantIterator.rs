macro_rules! EnumVariantIterator {
    () => {
        struct EnumVariantIterator < 'a > { variants : & 'a [EnumVariant] , idx : usize , }
    };
}

EnumVariantIterator!()
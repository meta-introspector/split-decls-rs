macro_rules! deps {
    () => {
        EnumVariantIterator!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > Iterator for EnumVariantIterator < 'a > { type Item = (Vec < TokenTree > , & 'a EnumVariant) ; fn next (& mut self) -> Option < Self :: Item > { let idx = self . idx ; let variant = self . variants . get (self . idx) ? ; self . idx += 1 ; let tokens = vec ! [TokenTree :: Literal (Literal :: u32_suffixed (idx as u32))] ; Some ((tokens , variant)) } }
    };
}

impl_10!();
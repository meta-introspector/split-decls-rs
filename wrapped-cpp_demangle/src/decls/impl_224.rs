macro_rules! deps {
    () => {
        TemplateParam!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'a > Hash for & 'a TemplateParam { fn hash < H > (& self , state : & mut H) where H : Hasher , { let self_ref : & TemplateParam = * self ; let self_ptr = self_ref as * const TemplateParam ; self_ptr . hash (state) ; } }
    };
}

impl_224!()
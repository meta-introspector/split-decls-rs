macro_rules! deps {
    () => {
        TemplateArgs!();
        SubstitutionTable!();
        Type!();
        GetTemplateArgs!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl GetTemplateArgs for Type { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { Type :: VendorExtension (_ , Some (ref args) , _) | Type :: TemplateTemplate (_ , ref args) => { Some (args) } Type :: PointerTo (ref ty) | Type :: LvalueRef (ref ty) | Type :: RvalueRef (ref ty) => { ty . get_template_args (subs) } _ => None , } } }
    };
}

impl_162!()
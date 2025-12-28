macro_rules! deps {
    () => {
        GetTemplateArgs!();
        SubstitutionTable!();
        TemplateArgs!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl GetTemplateArgs for TypeHandle { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { subs . get_type (self) . and_then (| ty | ty . get_template_args (subs)) } }
    };
}

impl_159!()
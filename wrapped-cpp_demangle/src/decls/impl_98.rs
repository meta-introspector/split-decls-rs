macro_rules! deps {
    () => {
        GetTemplateArgs!();
        SubstitutionTable!();
        TemplateArgs!();
        NestedName!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl GetTemplateArgs for NestedName { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { NestedName :: Template (_ , _ , ref prefix) | NestedName :: TemplateExplicitObject (ref prefix , _) => prefix . get_template_args (subs) , _ => None , } } }
    };
}

impl_98!();
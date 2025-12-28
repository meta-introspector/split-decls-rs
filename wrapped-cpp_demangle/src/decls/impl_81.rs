macro_rules! deps {
    () => {
        GetTemplateArgs!();
        SubstitutionTable!();
        TemplateArgs!();
        Name!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl GetTemplateArgs for Name { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { Name :: UnscopedTemplate (_ , ref args) => Some (args) , Name :: Nested (ref nested) => nested . get_template_args (subs) , Name :: Local (ref local) => local . get_template_args (subs) , Name :: Unscoped (_) => None , } } }
    };
}

impl_81!()
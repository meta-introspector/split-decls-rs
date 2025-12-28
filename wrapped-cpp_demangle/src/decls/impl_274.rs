macro_rules! deps {
    () => {
        TemplateArgs!();
        GetTemplateArgs!();
        LocalName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl GetTemplateArgs for LocalName { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { LocalName :: Relative (_ , None , _) => None , LocalName :: Relative (_ , Some (ref name) , _) | LocalName :: Default (_ , _ , ref name) => { name . get_template_args (subs) } } } }
    };
}

impl_274!();
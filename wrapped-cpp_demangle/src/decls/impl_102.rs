macro_rules! deps {
    () => {
        TemplateArgs!();
        SubstitutionTable!();
        TemplateParam!();
        Decltype!();
        GetTemplateArgs!();
        Prefix!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl GetTemplateArgs for Prefix { fn get_template_args < 'a > (& 'a self , _ : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { Prefix :: Template (_ , ref args) => Some (args) , Prefix :: Unqualified (_) | Prefix :: Nested (_ , _) | Prefix :: TemplateParam (_) | Prefix :: Decltype (_) | Prefix :: DataMember (_ , _) => None , } } }
    };
}

impl_102!();
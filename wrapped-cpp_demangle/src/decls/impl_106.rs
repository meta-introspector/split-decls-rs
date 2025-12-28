macro_rules! deps {
    () => {
        Prefix!();
        SubstitutionTable!();
        GetTemplateArgs!();
        Substitutable!();
        NonSubstitution!();
        TemplateArgs!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl GetTemplateArgs for PrefixHandle { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { PrefixHandle :: BackReference (idx) => { if let Some (& Substitutable :: Prefix (ref p)) = subs . get (idx) { p . get_template_args (subs) } else { None } } PrefixHandle :: NonSubstitution (NonSubstitution (idx)) => { if let Some (& Substitutable :: Prefix (ref p)) = subs . get_non_substitution (idx) { p . get_template_args (subs) } else { None } } _ => None , } } }
    };
}

impl_106!();
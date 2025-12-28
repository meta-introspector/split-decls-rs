macro_rules! deps {
    () => {
        Declaration!();
        DependencyValidator!();
        ValidationError!();
        DeclarationValidator!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl DeclarationValidator for DependencyValidator { fn validate (& self , declaration : & Declaration) -> Result < () , ValidationError > { for _dep in & declaration . referenced_types { } Ok (()) } }
    };
}

impl_251!()
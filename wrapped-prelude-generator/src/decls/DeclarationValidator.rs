macro_rules! deps {
    () => {
        Declaration!();
        ValidationError!();
    };
}

macro_rules! DeclarationValidator {
    () => {
        deps!();
        pub trait DeclarationValidator { fn validate (& self , declaration : & Declaration) -> Result < () , ValidationError > ; }
    };
}

DeclarationValidator!();
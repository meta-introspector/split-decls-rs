macro_rules! TypeNs {
    () => {
        # [derive (Clone , PartialEq , Eq , Debug , Hash)] pub struct TypeNs < 'db > { env : Arc < TraitEnvironment < 'db > > , ty : Ty < 'db > , }
    };
}

TypeNs!();
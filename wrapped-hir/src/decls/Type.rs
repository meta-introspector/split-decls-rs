macro_rules! Type {
    () => {
        # [derive (Clone , PartialEq , Eq , Debug , Hash)] pub struct Type < 'db > { env : Arc < TraitEnvironment < 'db > > , ty : Ty < 'db > , }
    };
}

Type!();
macro_rules! Generics {
    () => {
        # [derive (Clone , Debug)] pub struct Generics { def : GenericDefId , params : Arc < GenericParams > , store : Arc < ExpressionStore > , parent_generics : Option < Box < Generics > > , has_trait_self_param : bool , }
    };
}

Generics!()
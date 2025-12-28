macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! TraitObjectSyntax {
    () => {
        deps!();
        # [doc = " Syntax used to declare a trait object."] # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] # [repr (u8)] pub enum TraitObjectSyntax { Dyn = 0 , None = 1 , }
    };
}

TraitObjectSyntax!();
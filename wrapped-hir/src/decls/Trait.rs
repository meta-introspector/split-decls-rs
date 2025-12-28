macro_rules! Trait {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Trait { pub (crate) id : TraitId , }
    };
}

Trait!()
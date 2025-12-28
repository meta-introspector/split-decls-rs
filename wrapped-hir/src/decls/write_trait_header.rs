macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! write_trait_header {
    () => {
        deps!();
        fn write_trait_header < 'db > (trait_ : & Trait , f : & mut HirFormatter < '_ , 'db > ,) -> Result < () , HirDisplayError > { write_visibility (trait_ . module (f . db) . id , trait_ . visibility (f . db) , f) ? ; let data = f . db . trait_signature (trait_ . id) ; if data . flags . contains (TraitFlags :: UNSAFE) { f . write_str ("unsafe ") ? ; } if data . flags . contains (TraitFlags :: AUTO) { f . write_str ("auto ") ? ; } write ! (f , "trait {}" , data . name . display (f . db , f . edition ())) ? ; write_generic_params (GenericDefId :: TraitId (trait_ . id) , f) ? ; Ok (()) }
    };
}

write_trait_header!();
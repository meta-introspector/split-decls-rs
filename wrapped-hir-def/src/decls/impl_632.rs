macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl TraitId { # [inline] pub fn trait_items (self , db : & dyn DefDatabase) -> & TraitItems { TraitItems :: query (db , self) } }
    };
}

impl_632!();
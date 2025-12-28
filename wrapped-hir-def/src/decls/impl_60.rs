macro_rules! impl_60 {
    () => {
        impl TraitId { # [inline] pub fn trait_items (self , db : & dyn DefDatabase) -> & TraitItems { TraitItems :: query (db , self) } }
    };
}

impl_60!()
macro_rules! deps {
    () => {
        SizedByDefault!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl SizedByDefault { fn is_sized_trait (self , trait_ : TraitId , db : & dyn DefDatabase) -> bool { match self { Self :: NotSized => false , Self :: Sized { anchor } => { let sized_trait = LangItem :: Sized . resolve_trait (db , anchor) ; Some (trait_) == sized_trait } } } }
    };
}

impl_514!()
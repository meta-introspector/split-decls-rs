macro_rules! deps {
    () => {
        Id!();
        Spec!();
    };
}

macro_rules! impl_895 {
    () => {
        deps!();
        # [doc = " Initialization"] impl < 'repo > Spec < 'repo > { # [doc = " Create a single specification which points to `id`."] pub fn from_id (id : Id < 'repo >) -> Self { Spec { inner : gix_revision :: Spec :: Include (id . inner) , path : None , repo : id . repo , first_ref : None , second_ref : None , } } }
    };
}

impl_895!()
macro_rules! all_super_traits {
    () => {
        # [doc = " Returns an iterator over the whole super trait hierarchy (including the"] # [doc = " trait itself)."] pub fn all_super_traits (db : & dyn DefDatabase , trait_ : TraitId) -> SmallVec < [TraitId ; 4] > { let mut result = smallvec ! [trait_] ; let mut i = 0 ; while let Some (& t) = result . get (i) { direct_super_traits_cb (db , t , | tt | { if ! result . contains (& tt) { result . push (tt) ; } }) ; i += 1 ; } result }
    };
}

all_super_traits!();
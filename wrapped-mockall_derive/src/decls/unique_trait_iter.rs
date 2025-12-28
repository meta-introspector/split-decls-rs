macro_rules! deps {
    () => {
        MockTrait!();
        HashSet!();
        AttrFormatter!();
    };
}

macro_rules! unique_trait_iter {
    () => {
        deps!();
        # [doc = " Filter out multiple copies of the same trait, even if they're implemented on"] # [doc = " different types.  But allow them if they have different attributes, which "] # [doc = " probably indicates that they aren't meant to be compiled together."] fn unique_trait_iter < 'a , I : Iterator < Item = & 'a MockTrait > > (i : I) -> impl Iterator < Item = & 'a MockTrait > { let mut hs = HashSet :: < (Path , Vec < Attribute >) > :: default () ; i . filter (move | mt | { let impl_attrs = AttrFormatter :: new (& mt . attrs) . async_trait (false) . doc (false) . format () ; let key = (mt . trait_path . clone () , impl_attrs) ; if hs . contains (& key) { false } else { hs . insert (key) ; true } }) }
    };
}

unique_trait_iter!()
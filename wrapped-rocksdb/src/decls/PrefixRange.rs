macro_rules! PrefixRange {
    () => {
        # [doc = " Representation of a range of keys starting with given prefix."] # [doc = ""] # [doc = " Can be used as argument of [`crate::ReadOptions::set_iterate_range`] method"] # [doc = " to set iterate bounds."] # [derive (Clone , Copy)] pub struct PrefixRange < K > (pub K) ;
    };
}

PrefixRange!()
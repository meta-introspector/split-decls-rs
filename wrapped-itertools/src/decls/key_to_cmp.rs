macro_rules! key_to_cmp {
    () => {
        # [inline] pub (crate) fn key_to_cmp < T , K , F > (mut key : F) -> impl FnMut (& T , & T) -> Ordering where F : FnMut (& T) -> K , K : Ord , { move | a , b | key (a) . cmp (& key (b)) }
    };
}

key_to_cmp!()
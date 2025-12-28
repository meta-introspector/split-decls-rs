macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! cmp_entry_with_name {
    () => {
        deps!();
        fn cmp_entry_with_name (a : & tree :: Entry , filename : & BStr , is_tree : bool) -> Ordering { let common = a . filename . len () . min (filename . len ()) ; a . filename [.. common] . cmp (& filename [.. common]) . then_with (| | { let a = a . filename . get (common) . or_else (| | a . mode . is_tree () . then_some (& b'/')) ; let b = filename . get (common) . or_else (| | is_tree . then_some (& b'/')) ; a . cmp (& b) }) }
    };
}

cmp_entry_with_name!()
macro_rules! next_prefix {
    () => {
        # [doc = " Returns lowest value following largest value with given prefix."] # [doc = ""] # [doc = " In other words, computes upper bound for a prefix scan over list of keys"] # [doc = " sorted in lexicographical order.  This means that a prefix scan can be"] # [doc = " expressed as range scan over a right-open `[prefix, next_prefix(prefix))`"] # [doc = " range."] # [doc = ""] # [doc = " For example, for prefix `foo` the function returns `fop`."] # [doc = ""] # [doc = " Returns `None` if there is no value which can follow value with given"] # [doc = " prefix.  This happens when prefix consists entirely of `'\\xff'` bytes (or is"] # [doc = " empty)."] fn next_prefix (prefix : & [u8]) -> Option < Vec < u8 > > { let ffs = prefix . iter () . rev () . take_while (| & & byte | byte == u8 :: MAX) . count () ; let next = & prefix [.. (prefix . len () - ffs)] ; if next . is_empty () { None } else { let mut next = next . to_vec () ; * next . last_mut () . unwrap () += 1 ; Some (next) } }
    };
}

next_prefix!();
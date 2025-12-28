macro_rules! deps {
    () => {
        EntryRef!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl Ord for EntryRef < '_ > { fn cmp (& self , b : & Self) -> Ordering { let a = self ; let common = a . filename . len () . min (b . filename . len ()) ; a . filename [.. common] . cmp (& b . filename [.. common]) . then_with (| | { let a = a . filename . get (common) . or_else (| | a . mode . is_tree () . then_some (& b'/')) ; let b = b . filename . get (common) . or_else (| | b . mode . is_tree () . then_some (& b'/')) ; a . cmp (& b) }) } }
    };
}

impl_142!()
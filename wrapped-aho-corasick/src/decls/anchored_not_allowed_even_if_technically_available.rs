macro_rules! deps {
    () => {
        AhoCorasickKind!();
        Anchored!();
        Input!();
        StartKind!();
        DFA!();
        AhoCorasick!();
    };
}

macro_rules! anchored_not_allowed_even_if_technically_available {
    () => {
        deps!();
        # [test] fn anchored_not_allowed_even_if_technically_available () { let ac = AhoCorasick :: builder () . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . start_kind (StartKind :: Unanchored) . build (& ["foo"]) . unwrap () ; assert ! (ac . try_find (Input :: new ("foo") . anchored (Anchored :: Yes)) . is_err ()) ; let ac = AhoCorasick :: builder () . kind (Some (AhoCorasickKind :: ContiguousNFA)) . start_kind (StartKind :: Unanchored) . build (& ["foo"]) . unwrap () ; assert ! (ac . try_find (Input :: new ("foo") . anchored (Anchored :: Yes)) . is_err ()) ; let ac = AhoCorasick :: builder () . kind (Some (AhoCorasickKind :: DFA)) . start_kind (StartKind :: Unanchored) . build (& ["foo"]) . unwrap () ; assert ! (ac . try_find (Input :: new ("foo") . anchored (Anchored :: Yes)) . is_err ()) ; }
    };
}

anchored_not_allowed_even_if_technically_available!();
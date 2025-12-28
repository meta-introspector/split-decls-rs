macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! next_if_rule {
    () => {
        deps!();
        pub (super) fn next_if_rule < 'a > (pairs : & mut Pairs < 'a , Rule > , rule : Rule) -> Option < Pair < 'a , Rule > > { if pairs . peek () . is_some_and (| pair | pair . as_rule () == rule) { Some (pairs . next () . unwrap ()) } else { None } }
    };
}

next_if_rule!();
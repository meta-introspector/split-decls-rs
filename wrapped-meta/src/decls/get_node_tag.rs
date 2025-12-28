macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! get_node_tag {
    () => {
        deps!();
        fn get_node_tag < 'i > (pairs : & mut Peekable < Pairs < 'i , Rule > > ,) -> (Pair < 'i , Rule > , Option < (String , Position < 'i >) >) { let pair_or_tag = pairs . next () . unwrap () ; if let Some (next_pair) = pairs . peek () { if next_pair . as_rule () == Rule :: assignment_operator { pairs . next () . unwrap () ; let pair = pairs . next () . unwrap () ; (pair , Some ((pair_or_tag . as_str () [1 ..] . to_string () , pair_or_tag . as_span () . start_pos () ,)) ,) } else { (pair_or_tag , None) } } else { (pair_or_tag , None) } }
    };
}

get_node_tag!();
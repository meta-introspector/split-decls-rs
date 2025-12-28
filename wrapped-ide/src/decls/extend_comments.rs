macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! extend_comments {
    () => {
        deps!();
        fn extend_comments (comment : ast :: Comment) -> Option < TextRange > { let prev = adj_comments (& comment , Direction :: Prev) ; let next = adj_comments (& comment , Direction :: Next) ; if prev != next { Some (TextRange :: new (prev . syntax () . text_range () . start () , next . syntax () . text_range () . end ())) } else { None } }
    };
}

extend_comments!();
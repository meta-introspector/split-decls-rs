macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn size_of_options () { let actual = std :: mem :: size_of :: < Options > () ; let limit = 160 ; assert ! (actual <= limit , "{actual} <= {limit}: size shouldn't change without us knowing (on windows, it's bigger)") ; } }
    };
}

tests!();
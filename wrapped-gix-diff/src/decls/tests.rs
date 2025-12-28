macro_rules! deps {
    () => {
        Change!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn size_of_change () { let actual = std :: mem :: size_of :: < Change > () ; assert ! (actual <= 48 , "{actual} <= 48: this type shouldn't grow without us knowing") ; } }
    };
}

tests!()
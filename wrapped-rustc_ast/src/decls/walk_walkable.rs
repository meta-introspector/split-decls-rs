macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! walk_walkable {
    () => {
        deps!();
        macro_rules ! walk_walkable { ($ visitor : expr , $ expr : expr ,) => { Walkable :: walk_ref ($ expr , $ visitor) } ; }
    };
}

walk_walkable!();
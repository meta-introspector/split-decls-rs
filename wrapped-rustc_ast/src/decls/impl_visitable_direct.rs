macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! impl_visitable_direct {
    () => {
        deps!();
        macro_rules ! impl_visitable_direct { (<$ lt : lifetime > $ ($ ty : ty ,) *) => { $ (impl_visitable ! (|&$ lt self : $ ty , visitor : & mut V , _extra : () | { Walkable :: walk_ref (self , visitor) }) ;) * } ; }
    };
}

impl_visitable_direct!()
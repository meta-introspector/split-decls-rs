macro_rules! Walkable {
    () => {
        pub (crate) trait Walkable < 'a , V : Visitor < 'a > > { # [must_use] fn walk_ref (& 'a self , visitor : & mut V) -> V :: Result ; }
    };
}

Walkable!()
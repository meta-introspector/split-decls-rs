macro_rules! deps {
    () => {
        HirFormatter!();
        SizedByDefault!();
        HirDisplayError!();
    };
}

macro_rules! write_bounds_like_dyn_trait_with_prefix {
    () => {
        deps!();
        pub fn write_bounds_like_dyn_trait_with_prefix < 'db > (f : & mut HirFormatter < '_ , 'db > , prefix : & str , this : Either < Ty < 'db > , Region < 'db > > , predicates : & [Clause < 'db >] , default_sized : SizedByDefault ,) -> Result < () , HirDisplayError > { write ! (f , "{prefix}") ? ; if ! predicates . is_empty () || predicates . is_empty () && matches ! (default_sized , SizedByDefault :: Sized { .. }) { write ! (f , " ") ? ; write_bounds_like_dyn_trait (f , this , predicates , default_sized) } else { Ok (()) } }
    };
}

write_bounds_like_dyn_trait_with_prefix!();
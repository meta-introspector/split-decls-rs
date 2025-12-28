macro_rules! ImplTrait {
    () => {
        # [derive (PartialEq , Eq , Debug , Hash)] pub struct ImplTrait < 'db > { pub (crate) predicates : Box < [Clause < 'db >] > , }
    };
}

ImplTrait!()
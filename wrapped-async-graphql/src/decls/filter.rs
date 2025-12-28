macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! filter {
    () => {
        deps!();
        fn filter < 'a > (fields : & mut Vec < & 'a Field > , fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , selection_set : & 'a SelectionSet , name : & str ,) { for item in & selection_set . items { match & item . node { Selection :: Field (field) => { if field . node . name . node == name { fields . push (& field . node) } } Selection :: InlineFragment (fragment) => { filter (fields , fragments , & fragment . node . selection_set . node , name) } Selection :: FragmentSpread (spread) => { if let Some (fragment) = fragments . get (& spread . node . fragment_name . node) { filter (fields , fragments , & fragment . node . selection_set . node , name) } } } } }
    };
}

filter!();
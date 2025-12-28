macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! SelectionFieldsIter {
    () => {
        deps!();
        struct SelectionFieldsIter < 'a > { fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , iter : Vec < std :: slice :: Iter < 'a , Positioned < Selection > > > , context : & 'a Context < 'a > , }
    };
}

SelectionFieldsIter!()
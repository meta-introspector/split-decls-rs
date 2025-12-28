macro_rules! deps {
    () => {
        Field!();
        Context!();
    };
}

macro_rules! SelectionField {
    () => {
        deps!();
        # [doc = " Selection field."] # [derive (Clone , Copy)] pub struct SelectionField < 'a > { pub (crate) fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , pub (crate) field : & 'a Field , pub (crate) context : & 'a Context < 'a > , }
    };
}

SelectionField!()
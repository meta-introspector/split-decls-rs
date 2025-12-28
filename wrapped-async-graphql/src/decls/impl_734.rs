macro_rules! deps {
    () => {
        EdgeNameType!();
        DefaultEdgeName!();
        Edge!();
        OutputType!();
    };
}

macro_rules! impl_734 {
    () => {
        deps!();
        impl EdgeNameType for DefaultEdgeName { fn type_name < T : OutputType > () -> String { format ! ("{}Edge" , T :: type_name ()) } }
    };
}

impl_734!()
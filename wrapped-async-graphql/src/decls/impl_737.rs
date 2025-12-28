macro_rules! deps {
    () => {
        Connection!();
        ConnectionNameType!();
        OutputType!();
        DefaultConnectionName!();
    };
}

macro_rules! impl_737 {
    () => {
        deps!();
        impl ConnectionNameType for DefaultConnectionName { fn type_name < T : OutputType > () -> String { format ! ("{}Connection" , T :: type_name ()) } }
    };
}

impl_737!()
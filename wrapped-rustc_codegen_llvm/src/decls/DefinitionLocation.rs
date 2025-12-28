macro_rules! DefinitionLocation {
    () => {
        pub (crate) type DefinitionLocation < 'll > = (& 'll DIFile , c_uint) ;
    };
}

DefinitionLocation!()
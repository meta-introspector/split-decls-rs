macro_rules! deps {
    () => {
        Dog!();
        Interface!();
        Cat!();
    };
}

macro_rules! Pet {
    () => {
        deps!();
        # [derive (Interface)] # [graphql (internal , field (name = "name" , ty = "Option<String>" , arg (name = "surname" , ty = "Option<bool>")))] enum Pet { Dog (Dog) , Cat (Cat) , }
    };
}

Pet!()
macro_rules! deps {
    () => {
        Interface!();
        Dog!();
        Cat!();
    };
}

macro_rules! Pet {
    () => {
        deps!();
        # [derive (Interface)] # [graphql (internal , field (name = "name" , ty = "Option<String>" , arg (name = "surname" , ty = "Option<bool>")))] enum Pet { Dog (Dog) , Cat (Cat) , }
    };
}

Pet!();
macro_rules! deps {
    () => {
        Interface!();
        Dog!();
    };
}

macro_rules! Canine {
    () => {
        deps!();
        # [derive (Interface)] # [graphql (internal , field (name = "name" , ty = "Option<String>" , arg (name = "surname" , ty = "Option<bool>")))] enum Canine { Dog (Dog) , }
    };
}

Canine!()
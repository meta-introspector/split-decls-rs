macro_rules! deps {
    () => {
        Interface!();
        Human!();
        Alien!();
        Cat!();
        Dog!();
    };
}

macro_rules! Being {
    () => {
        deps!();
        # [derive (Interface)] # [graphql (internal , field (name = "name" , ty = "Option<String>" , arg (name = "surname" , ty = "Option<bool>")))] enum Being { Dog (Dog) , Cat (Cat) , Human (Human) , Alien (Alien) , }
    };
}

Being!()
macro_rules! deps {
    () => {
        Alien!();
        Human!();
        Interface!();
    };
}

macro_rules! Intelligent {
    () => {
        deps!();
        # [derive (Interface)] # [graphql (internal , field (name = "iq" , ty = "Option<i32>"))] enum Intelligent { Human (Human) , Alien (Alien) , }
    };
}

Intelligent!()
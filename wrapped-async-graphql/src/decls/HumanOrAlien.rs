macro_rules! deps {
    () => {
        Human!();
        Union!();
        Alien!();
    };
}

macro_rules! HumanOrAlien {
    () => {
        deps!();
        # [derive (Union)] # [graphql (internal)] enum HumanOrAlien { Human (Human) , Alien (Alien) , }
    };
}

HumanOrAlien!()
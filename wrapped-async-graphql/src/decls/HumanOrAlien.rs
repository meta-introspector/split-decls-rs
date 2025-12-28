macro_rules! deps {
    () => {
        Union!();
        Human!();
        Alien!();
    };
}

macro_rules! HumanOrAlien {
    () => {
        deps!();
        # [derive (Union)] # [graphql (internal)] enum HumanOrAlien { Human (Human) , Alien (Alien) , }
    };
}

HumanOrAlien!();
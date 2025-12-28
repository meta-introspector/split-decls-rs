macro_rules! deps {
    () => {
        Human!();
        Union!();
        Dog!();
    };
}

macro_rules! DogOrHuman {
    () => {
        deps!();
        # [derive (Union)] # [graphql (internal)] enum DogOrHuman { Dog (Dog) , Human (Human) , }
    };
}

DogOrHuman!()
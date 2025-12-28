macro_rules! deps {
    () => {
        Cat!();
        Dog!();
        Union!();
    };
}

macro_rules! CatOrDog {
    () => {
        deps!();
        # [derive (Union)] # [graphql (internal)] enum CatOrDog { Cat (Cat) , Dog (Dog) , }
    };
}

CatOrDog!();
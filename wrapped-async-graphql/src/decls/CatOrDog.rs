macro_rules! deps {
    () => {
        Union!();
        Cat!();
        Dog!();
    };
}

macro_rules! CatOrDog {
    () => {
        deps!();
        # [derive (Union)] # [graphql (internal)] enum CatOrDog { Cat (Cat) , Dog (Dog) , }
    };
}

CatOrDog!()
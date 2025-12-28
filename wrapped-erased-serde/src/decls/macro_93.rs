macro_rules! deps {
    () => {
        Serialize!();
    };
}

macro_rules! macro_93 {
    () => {
        deps!();
        serialize_trait_object ! (Serialize) ;
    };
}

macro_93!();
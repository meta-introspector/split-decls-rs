macro_rules! deps {
    () => {
        Throughput!();
        Value!();
    };
}

macro_rules! ProgressFormat {
    () => {
        deps!();
        struct ProgressFormat < 'a > (& 'a Option < Value > , u16 , Option < unit :: display :: Throughput >) ;
    };
}

ProgressFormat!()
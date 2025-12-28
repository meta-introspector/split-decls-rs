macro_rules! deps {
    () => {
        GeneratorError!();
    };
}

macro_rules! GeneratorResult {
    () => {
        deps!();
        pub type GeneratorResult < T > = std :: result :: Result < T , GeneratorError > ;
    };
}

GeneratorResult!()
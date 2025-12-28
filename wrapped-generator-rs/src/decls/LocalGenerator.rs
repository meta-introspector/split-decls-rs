macro_rules! deps {
    () => {
        GeneratorObj!();
    };
}

macro_rules! LocalGenerator {
    () => {
        deps!();
        # [doc = " the local generator type, can't Send"] pub type LocalGenerator < 'a , A , T > = GeneratorObj < 'a , A , T , true > ;
    };
}

LocalGenerator!();
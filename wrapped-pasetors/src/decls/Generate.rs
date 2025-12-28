macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Generate {
    () => {
        deps!();
        # [doc = " A type `T` that can be generated for a given version `V`."] pub trait Generate < T , V : Version > { # [doc = " Generate `T`."] fn generate () -> Result < T , Error > ; }
    };
}

Generate!();
macro_rules! Reduce {
    () => {
        # [doc = " Modular reduction from a larger value `T`."] # [doc = ""] # [doc = " This can be seen as fixed modular reduction, where the modulus is fixed at compile time"] # [doc = " by `Self`."] # [doc = ""] # [doc = " For modular reduction with a variable modulus, use [`Rem`]."] pub trait Reduce < T > : Sized { # [doc = " Reduces `self` modulo `Modulus`."] fn reduce (value : & T) -> Self ; }
    };
}

Reduce!()
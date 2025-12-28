macro_rules! deps {
    () => {
        Pow!();
    };
}

macro_rules! MultiExponentiateBoundedExp {
    () => {
        deps!();
        # [doc = " Performs modular multi-exponentiation using Montgomery's ladder."] # [doc = " `exponent_bits` represents the number of bits to take into account for the exponent."] # [doc = ""] # [doc = " See: Straus, E. G. Problems and solutions: Addition chains of vectors. American Mathematical Monthly 71 (1964), 806–808."] # [doc = ""] # [doc = " NOTE: this value is leaked in the time pattern."] pub trait MultiExponentiateBoundedExp < Exponent , BasesAndExponents > : Pow < Exponent > + Sized where BasesAndExponents : AsRef < [(Self , Exponent)] > + ? Sized , { # [doc = " Calculates `x1 ^ k1 * ... * xn ^ kn`."] fn multi_exponentiate_bounded_exp (bases_and_exponents : & BasesAndExponents , exponent_bits : u32 ,) -> Self ; }
    };
}

MultiExponentiateBoundedExp!()
macro_rules! deps {
    () => {
        Pow!();
    };
}

macro_rules! MultiExponentiate {
    () => {
        deps!();
        # [doc = " Performs modular multi-exponentiation using Montgomery's ladder."] # [doc = ""] # [doc = " See: Straus, E. G. Problems and solutions: Addition chains of vectors. American Mathematical Monthly 71 (1964), 806–808."] pub trait MultiExponentiate < Exponent , BasesAndExponents > : Pow < Exponent > + Sized where BasesAndExponents : AsRef < [(Self , Exponent)] > + ? Sized , { # [doc = " Calculates `x1 ^ k1 * ... * xn ^ kn`."] fn multi_exponentiate (bases_and_exponents : & BasesAndExponents) -> Self ; }
    };
}

MultiExponentiate!()
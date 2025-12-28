macro_rules! deps {
    () => {
        One!();
        Zero!();
    };
}

macro_rules! JacobiSymbol {
    () => {
        deps!();
        # [doc = " Possible return values for Jacobi symbol calculations."] # [derive (Debug , Copy , Clone)] # [repr (i8)] pub enum JacobiSymbol { # [doc = " The two arguments are not coprime, they have a common divisor apart from 1."] Zero = 0 , # [doc = " The two arguments are coprime. If the lower argument is prime, then the upper argument"] # [doc = " is quadratic residue modulo the lower argument. Otherwise, the upper argument is known to"] # [doc = " be quadratic nonresidue for an even number of prime factors of the lower argument."] One = 1 , # [doc = " The two terms are coprime, and the upper argument is a quadratic nonresidue modulo the"] # [doc = " lower argument."] MinusOne = - 1 , }
    };
}

JacobiSymbol!();
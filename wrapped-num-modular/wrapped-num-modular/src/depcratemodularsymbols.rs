// Generated macro for ModularSymbols (trait)
macro_rules! DepcrateModularSymbols {
() => {
// Module: crate
// Provides: {"ModularSymbols"}
// Dependencies: {}
# [doc = " Math symbols related to modular arithmetics"] pub trait ModularSymbols < Modulus = Self > { # [doc = " Calculate Legendre Symbol (a|n), where a is `self`."] # [doc = ""] # [doc = " Note that this function doesn't perform a full primality check, since"] # [doc = " is costly. So if n is not a prime, the result can be not reasonable."] # [doc = ""] # [doc = " # Panics"] # [doc = " Only if n is not prime"] # [inline] fn legendre (& self , n : Modulus) -> i8 { self . checked_legendre (n) . expect ("n shoud be a prime") } # [doc = " Calculate Legendre Symbol (a|n), where a is `self`. Returns [None] only if n is"] # [doc = " not a prime."] # [doc = ""] # [doc = " Note that this function doesn't perform a full primality check, since"] # [doc = " is costly. So if n is not a prime, the result can be not reasonable."] # [doc = ""] # [doc = " # Panics"] # [doc = " Only if n is not prime"] fn checked_legendre (& self , n : Modulus) -> Option < i8 > ; # [doc = " Calculate Jacobi Symbol (a|n), where a is `self`"] # [doc = ""] # [doc = " # Panics"] # [doc = " if n is negative or even"] # [inline] fn jacobi (& self , n : Modulus) -> i8 { self . checked_jacobi (n) . expect ("the Jacobi symbol is only defined for non-negative odd integers") } # [doc = " Calculate Jacobi Symbol (a|n), where a is `self`. Returns [None] if n is negative or even."] fn checked_jacobi (& self , n : Modulus) -> Option < i8 > ; # [doc = " Calculate Kronecker Symbol (a|n), where a is `self`"] fn kronecker (& self , n : Modulus) -> i8 ; }
};
}

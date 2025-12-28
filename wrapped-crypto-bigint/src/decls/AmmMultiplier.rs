macro_rules! deps {
    () => {
        Monty!();
        MontyMultiplier!();
        Integer!();
    };
}

macro_rules! AmmMultiplier {
    () => {
        deps!();
        # [doc = " Prepared Montgomery multiplier for tight loops, performing \"Almost Montgomery Multiplication\"."] # [doc = ""] # [doc = " NOTE: the resulting output of any of these functions will be reduced to the *bit length* of the"] # [doc = " modulus, but not fully reduced and may exceed the modulus. A final reduction is required to"] # [doc = " ensure AMM results are fully reduced, and should not be exposed outside the internals of this"] # [doc = " crate."] pub (crate) trait AmmMultiplier < 'a > : MontyMultiplier < 'a > { # [doc = " Perform an \"Almost Montgomery Multiplication\", assigning the product to `a`."] fn mul_amm_assign (& mut self , a : & mut < Self :: Monty as Monty > :: Integer , b : & < Self :: Monty as Monty > :: Integer ,) ; # [doc = " Perform a squaring using \"Almost Montgomery Multiplication\", assigning the result to `a`."] fn square_amm_assign (& mut self , a : & mut < Self :: Monty as Monty > :: Integer) ; }
    };
}

AmmMultiplier!()
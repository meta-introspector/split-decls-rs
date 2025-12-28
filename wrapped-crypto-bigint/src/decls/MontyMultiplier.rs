macro_rules! deps {
    () => {
        Monty!();
    };
}

macro_rules! MontyMultiplier {
    () => {
        deps!();
        # [doc = " Prepared Montgomery multiplier for tight loops."] # [doc = ""] # [doc = " Allows one to perform inplace multiplication without allocations"] # [doc = " (important for the `BoxedUint` case)."] # [doc = ""] # [doc = " NOTE: You will be operating with Montgomery representations directly,"] # [doc = " make sure they all correspond to the same set of parameters."] pub trait MontyMultiplier < 'a > : From < & 'a < Self :: Monty as Monty > :: Params > { # [doc = " The associated Montgomery-representation integer."] type Monty : Monty ; # [doc = " Performs a Montgomery multiplication, assigning a fully reduced result to `lhs`."] fn mul_assign (& mut self , lhs : & mut Self :: Monty , rhs : & Self :: Monty) ; # [doc = " Performs a Montgomery squaring, assigning a fully reduced result to `lhs`."] fn square_assign (& mut self , lhs : & mut Self :: Monty) ; }
    };
}

MontyMultiplier!()
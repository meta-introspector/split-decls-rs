macro_rules! deps {
    () => {
        ArrayLength!();
        GenericSequence!();
    };
}

macro_rules! Concat {
    () => {
        deps!();
        # [doc = " Defines `GenericSequence`s which can be joined together, forming a larger array."] # [doc = ""] # [doc = " # Safety"] # [doc = " While the [`concat`](Concat::concat) method is marked safe,"] # [doc = " care must be taken when implementing it."] pub unsafe trait Concat < T , M : ArrayLength > : GenericSequence < T > { # [doc = " Sequence to be concatenated with `self`"] type Rest : GenericSequence < T , Length = M > ; # [doc = " Resulting sequence formed by the concatenation."] type Output : GenericSequence < T > ; # [doc = " Concatenate, or join, two sequences."] fn concat (self , rest : Self :: Rest) -> Self :: Output ; }
    };
}

Concat!()
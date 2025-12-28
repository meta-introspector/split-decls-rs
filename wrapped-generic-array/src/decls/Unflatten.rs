macro_rules! deps {
    () => {
        ArrayLength!();
        GenericSequence!();
        GenericArray!();
    };
}

macro_rules! Unflatten {
    () => {
        deps!();
        # [doc = " Defines a `GenericSequence` of `T` which can be split evenly into a sequence of `GenericArray`s,"] # [doc = ""] # [doc = " # Safety"] # [doc = " While the [`unflatten`](Unflatten::unflatten) method is marked safe,"] # [doc = " care must be taken when implementing it. However, the given trait bounds"] # [doc = " should be sufficient to ensure safety."] pub unsafe trait Unflatten < T , NM , N > : GenericSequence < T , Length = NM > where NM : ArrayLength + Div < N > , N : ArrayLength , Quot < NM , N > : ArrayLength , { # [doc = " Unflattened sequence type"] type Output : GenericSequence < GenericArray < T , N > , Length = Quot < NM , N > > ; # [doc = " Unflattens the sequence into a sequence of `GenericArray`s."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use generic_array::{arr, sequence::Unflatten};"] # [doc = " assert_eq!("] # [doc = "     arr![1, 2, 3, 4, 5, 6].unflatten(),"] # [doc = "     arr![arr![1, 2], arr![3, 4], arr![5, 6]]"] # [doc = " );"] # [doc = " ```"] fn unflatten (self) -> Self :: Output ; }
    };
}

Unflatten!();
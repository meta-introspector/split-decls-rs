macro_rules! deps {
    () => {
        GenericSequence!();
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! Flatten {
    () => {
        deps!();
        # [doc = " Defines a `GenericSequence` of `GenericArray`s which can be flattened into a single `GenericArray`,"] # [doc = " at zero cost."] # [doc = ""] # [doc = " # Safety"] # [doc = " While the [`flatten`](Flatten::flatten) method is marked safe,"] # [doc = " care must be taken when implementing it. However, the given trait bounds"] # [doc = " should be sufficient to ensure safety."] pub unsafe trait Flatten < T , N , M > : GenericSequence < GenericArray < T , N > , Length = M > where N : ArrayLength + Mul < M > , Prod < N , M > : ArrayLength , { # [doc = " Flattened sequence type"] type Output : GenericSequence < T , Length = Prod < N , M > > ; # [doc = " Flattens the sequence into a single `GenericArray`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use generic_array::{arr, sequence::Flatten};"] # [doc = " assert_eq!("] # [doc = "     arr![arr![1, 2], arr![3, 4], arr![5, 6]].flatten(),"] # [doc = "     arr![1, 2, 3, 4, 5, 6]"] # [doc = " );"] # [doc = " ```"] fn flatten (self) -> Self :: Output ; }
    };
}

Flatten!()
macro_rules! deps {
    () => {
        GenericSequence!();
    };
}

macro_rules! FallibleGenericSequence {
    () => {
        deps!();
        # [doc = " Extension to `GenericSequence` for fallible initialization."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Care must be taken when implementing such that methods are safe."] # [doc = ""] # [doc = " Lengths must match, and element drop on panic or error must be handled."] pub unsafe trait FallibleGenericSequence < T > : GenericSequence < T > { # [doc = " Initializes a new sequence instance using the given fallible function."] # [doc = ""] # [doc = " If the generator function returns an error or panics while initializing the sequence,"] # [doc = " any already initialized elements will be dropped and the error returned."] fn try_generate < F , E > (f : F) -> Result < Self :: Sequence , E > where F : FnMut (usize) -> Result < T , E > ; # [doc = " Initializes a new sequence instance from a fallible iterator."] # [doc = ""] # [doc = " If the iterator returns an error or panics while initializing the sequence,"] # [doc = " any already initialized elements will be dropped and the error returned."] # [doc = ""] # [doc = " This is equivalent to `iter.collect::<Result<GenericArray<T, N>, E>>()` _except_"] # [doc = " it won't panic due to `Result::from_iter` truncating the underlying iterator"] # [doc = " if an error occurs, leading to a length mismatch."] fn from_fallible_iter < I , E > (iter : I) -> Result < Self :: Sequence , E > where I : IntoIterator < Item = Result < T , E > > ; }
    };
}

FallibleGenericSequence!()
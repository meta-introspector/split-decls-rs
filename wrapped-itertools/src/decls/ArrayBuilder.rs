macro_rules! ArrayBuilder {
    () => {
        # [doc = " An array of at most `N` elements."] struct ArrayBuilder < T , const N : usize > { # [doc = " The (possibly uninitialized) elements of the `ArrayBuilder`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The elements of `arr[..len]` are valid `T`s."] arr : [MaybeUninit < T > ; N] , # [doc = " The number of leading elements of `arr` that are valid `T`s, len <= N."] len : usize , }
    };
}

ArrayBuilder!();
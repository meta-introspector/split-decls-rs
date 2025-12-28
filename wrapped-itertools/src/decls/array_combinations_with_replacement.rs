macro_rules! deps {
    () => {
        ArrayCombinationsWithReplacement!();
    };
}

macro_rules! array_combinations_with_replacement {
    () => {
        deps!();
        # [doc = " Create a new `ArrayCombinationsWithReplacement`` from a cloneable iterator."] pub fn array_combinations_with_replacement < I : Iterator , const K : usize > (iter : I ,) -> ArrayCombinationsWithReplacement < I , K > where I :: Item : Clone , { ArrayCombinationsWithReplacement :: new (iter , [0 ; K]) }
    };
}

array_combinations_with_replacement!();
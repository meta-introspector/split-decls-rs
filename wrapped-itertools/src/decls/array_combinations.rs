macro_rules! deps {
    () => {
        ArrayCombinations!();
    };
}

macro_rules! array_combinations {
    () => {
        deps!();
        # [doc = " Create a new `ArrayCombinations` from a cloneable iterator."] pub fn array_combinations < I : Iterator , const K : usize > (iter : I) -> ArrayCombinations < I , K > where I :: Item : Clone , { ArrayCombinations :: new (iter , array :: from_fn (| i | i)) }
    };
}

array_combinations!()
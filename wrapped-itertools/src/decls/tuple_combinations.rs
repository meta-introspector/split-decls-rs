macro_rules! deps {
    () => {
        HasCombination!();
        TupleCombinations!();
    };
}

macro_rules! tuple_combinations {
    () => {
        deps!();
        # [doc = " Create a new `TupleCombinations` from a cloneable iterator."] pub fn tuple_combinations < T , I > (iter : I) -> TupleCombinations < I , T > where I : Iterator , I :: Item : Clone , T : HasCombination < I > , { TupleCombinations { iter : T :: Combination :: from (iter) , _mi : PhantomData , } }
    };
}

tuple_combinations!();
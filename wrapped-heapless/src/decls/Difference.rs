macro_rules! deps {
    () => {
        Iter!();
        IndexSet!();
    };
}

macro_rules! Difference {
    () => {
        deps!();
        # [doc = " An iterator over the difference of two `IndexSet`s."] # [doc = ""] # [doc = " This is created by the [`IndexSet::difference`] method."] pub struct Difference < 'a , T , S , const N : usize > where S : BuildHasher , T : Eq + Hash , { iter : Iter < 'a , T > , other : & 'a IndexSet < T , S , N > , }
    };
}

Difference!()
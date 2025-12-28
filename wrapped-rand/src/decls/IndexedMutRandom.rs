macro_rules! deps {
    () => {
        Weight!();
        SampleBorrow!();
        Distribution!();
        Rng!();
        IndexedRandom!();
        WeightedIndex!();
        SampleUniform!();
    };
}

macro_rules! IndexedMutRandom {
    () => {
        deps!();
        # [doc = " Extension trait on indexable lists, providing random sampling methods."] # [doc = ""] # [doc = " This trait is implemented automatically for every type implementing"] # [doc = " [`IndexedRandom`] and [`std::ops::IndexMut<usize>`]."] pub trait IndexedMutRandom : IndexedRandom + IndexMut < usize > { # [doc = " Uniformly sample one element (mut)"] # [doc = ""] # [doc = " Returns a mutable reference to one uniformly-sampled random element of"] # [doc = " the slice, or `None` if the slice is empty."] # [doc = ""] # [doc = " For slices, complexity is `O(1)`."] fn choose_mut < R > (& mut self , rng : & mut R) -> Option < & mut Self :: Output > where R : Rng + ? Sized , { if self . is_empty () { None } else { let len = self . len () ; Some (& mut self [rng . random_range (.. len)]) } } # [doc = " Biased sampling for one element (mut)"] # [doc = ""] # [doc = " Returns a mutable reference to one element of the slice, sampled according"] # [doc = " to the provided weights. Returns `None` only if the slice is empty."] # [doc = ""] # [doc = " The specified function `weight` maps each item `x` to a relative"] # [doc = " likelihood `weight(x)`. The probability of each item being selected is"] # [doc = " therefore `weight(x) / s`, where `s` is the sum of all `weight(x)`."] # [doc = ""] # [doc = " For slices of length `n`, complexity is `O(n)`."] # [doc = " For more information about the underlying algorithm,"] # [doc = " see the [`WeightedIndex`] distribution."] # [doc = ""] # [doc = " See also [`choose_weighted`]."] # [doc = ""] # [doc = " [`choose_mut`]: IndexedMutRandom::choose_mut"] # [doc = " [`choose_weighted`]: IndexedRandom::choose_weighted"] # [doc = " [`WeightedIndex`]: crate::distr::weighted::WeightedIndex"] # [cfg (feature = "alloc")] fn choose_weighted_mut < R , F , B , X > (& mut self , rng : & mut R , weight : F ,) -> Result < & mut Self :: Output , WeightError > where R : Rng + ? Sized , F : Fn (& Self :: Output) -> B , B : SampleBorrow < X > , X : SampleUniform + Weight + PartialOrd < X > , { use crate :: distr :: { Distribution , weighted :: WeightedIndex } ; let distr = WeightedIndex :: new ((0 .. self . len ()) . map (| idx | weight (& self [idx]))) ? ; let index = distr . sample (rng) ; Ok (& mut self [index]) } }
    };
}

IndexedMutRandom!();
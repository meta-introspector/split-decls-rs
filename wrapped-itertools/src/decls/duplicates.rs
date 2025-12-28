macro_rules! deps {
    () => {
        Duplicates!();
    };
}

macro_rules! duplicates {
    () => {
        deps!();
        # [doc = " Create a new `Duplicates` iterator."] pub fn duplicates < I > (iter : I) -> Duplicates < I > where I : Iterator , I :: Item : Eq + Hash , { Duplicates :: new (iter , private :: ById) }
    };
}

duplicates!()
macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " Iterator over the labeled data"] pub struct Iter < 'a , A > where A : Float , { fences : (A , A , A , A) , iter : slice :: Iter < 'a , A > , }
    };
}

Iter!();
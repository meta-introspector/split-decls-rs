macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! sum {
    () => {
        deps!();
        fn sum < A > (xs : & [A]) -> A where A : Float , { use std :: ops :: Add ; xs . iter () . cloned () . fold (A :: cast (0) , Add :: add) }
    };
}

sum!()
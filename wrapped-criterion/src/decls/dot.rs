macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! dot {
    () => {
        deps!();
        fn dot < A > (xs : & [A] , ys : & [A]) -> A where A : Float , { xs . iter () . zip (ys) . fold (A :: cast (0) , | acc , (& x , & y) | acc + x * y) }
    };
}

dot!();
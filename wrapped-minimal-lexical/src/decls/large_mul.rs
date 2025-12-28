macro_rules! deps {
    () => {
        VecType!();
        Limb!();
    };
}

macro_rules! large_mul {
    () => {
        deps!();
        # [doc = " Multiply bigint by bigint using grade-school multiplication algorithm."] # [inline (always)] pub fn large_mul (x : & mut VecType , y : & [Limb]) -> Option < () > { if y . len () == 1 { small_mul (x , y [0]) ? ; } else { * x = long_mul (y , x) ? ; } Some (()) }
    };
}

large_mul!();
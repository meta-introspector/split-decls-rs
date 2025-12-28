macro_rules! Round {
    () => {
        # [doc = " IEEE 754 rounding mode, excluding the optional `roundTiesToAway` version of nearest."] # [doc = ""] # [doc = " Integer representation comes from what CORE-MATH uses for indexing."] # [cfg_attr (not (feature = "unstable-public-internals") , allow (dead_code))] # [derive (Clone , Copy , Debug , PartialEq)] pub enum Round { # [doc = " IEEE 754 nearest, `roundTiesToEven`."] Nearest = 0 , # [doc = " IEEE 754 `roundTowardNegative`."] Negative = 1 , # [doc = " IEEE 754 `roundTowardPositive`."] Positive = 2 , # [doc = " IEEE 754 `roundTowardZero`."] Zero = 3 , }
    };
}

Round!();
macro_rules! Round {
    () => {
        # [doc = " IEEE-754R 4.3: Rounding-direction attributes."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Round { NearestTiesToEven , TowardPositive , TowardNegative , TowardZero , NearestTiesToAway , }
    };
}

Round!()
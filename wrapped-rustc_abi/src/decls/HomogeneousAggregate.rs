macro_rules! HomogeneousAggregate {
    () => {
        # [doc = " Return value from the `homogeneous_aggregate` test function."] # [derive (Copy , Clone , Debug)] pub enum HomogeneousAggregate { # [doc = " Yes, all the \"leaf fields\" of this struct are passed in the"] # [doc = " same way (specified in the `Reg` value)."] Homogeneous (Reg) , # [doc = " There are no leaf fields at all."] NoData , }
    };
}

HomogeneousAggregate!();
macro_rules! IntPredicate {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum IntPredicate { IntEQ , IntNE , IntUGT , IntUGE , IntULT , IntULE , IntSGT , IntSGE , IntSLT , IntSLE , }
    };
}

IntPredicate!();
macro_rules! deps {
    () => {
        Effects!();
    };
}

macro_rules! EffectIndexIter {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct EffectIndexIter { index : usize , effects : Effects , }
    };
}

EffectIndexIter!()
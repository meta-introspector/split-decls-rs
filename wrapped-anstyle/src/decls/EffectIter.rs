macro_rules! deps {
    () => {
        Effects!();
    };
}

macro_rules! EffectIter {
    () => {
        deps!();
        # [doc = " Enumerate each enabled value in [`Effects`]"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct EffectIter { index : usize , effects : Effects , }
    };
}

EffectIter!();
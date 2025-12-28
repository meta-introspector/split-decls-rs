macro_rules! deps {
    () => {
        Effects!();
    };
}

macro_rules! EffectsDisplay {
    () => {
        deps!();
        # [derive (Copy , Clone , Default , Debug)] struct EffectsDisplay (Effects) ;
    };
}

EffectsDisplay!()
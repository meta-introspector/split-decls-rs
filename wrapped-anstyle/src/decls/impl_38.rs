macro_rules! deps {
    () => {
        Effects!();
        EffectIter!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Iterator for EffectIter { type Item = Effects ; fn next (& mut self) -> Option < Self :: Item > { while self . index < METADATA . len () { let index = self . index ; self . index += 1 ; let effect = Effects (1 << index) ; if self . effects . contains (effect) { return Some (effect) ; } } None } }
    };
}

impl_38!();
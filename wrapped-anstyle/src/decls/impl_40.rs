macro_rules! deps {
    () => {
        EffectIndexIter!();
        Effects!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Iterator for EffectIndexIter { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { while self . index < METADATA . len () { let index = self . index ; self . index += 1 ; let effect = Effects (1 << index) ; if self . effects . contains (effect) { return Some (index) ; } } None } }
    };
}

impl_40!();
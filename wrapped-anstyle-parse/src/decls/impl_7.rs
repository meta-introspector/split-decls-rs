macro_rules! deps {
    () => {
        ParamsIter!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > Iterator for ParamsIter < 'a > { type Item = & 'a [u16] ; fn next (& mut self) -> Option < Self :: Item > { if self . index >= self . params . len () { return None ; } let num_subparams = self . params . subparams [self . index] ; let param = & self . params . params [self . index .. self . index + num_subparams as usize] ; self . index += num_subparams as usize ; Some (param) } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = self . params . len () - self . index ; (remaining , Some (remaining)) } }
    };
}

impl_7!();
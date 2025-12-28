macro_rules! deps {
    () => {
        Action!();
        Change!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [cfg (feature = "terminfo")] impl < T > Action < T > { pub fn actual_value (& self) -> Option < & T > { match self { Action :: Keep (val) | Action :: Change (val) => Some (val) , Action :: None => None , } } }
    };
}

impl_44!();
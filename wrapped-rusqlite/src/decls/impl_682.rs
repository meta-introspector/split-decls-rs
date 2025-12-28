macro_rules! deps {
    () => {
        Named!();
        SmallCString!();
        Result!();
        Name!();
    };
}

macro_rules! impl_682 {
    () => {
        deps!();
        impl Name for & str { fn as_cstr (& self) -> Result < Named < '_ > > { let ss = SmallCString :: new (self) ? ; Ok (Named :: Small (ss)) } }
    };
}

impl_682!();
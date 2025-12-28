macro_rules! Var {
    () => {
        # [derive (Debug)] struct Var < 'a > { name : Cow < 'a , str > , default : Option < Cow < 'a , str > > , }
    };
}

Var!();
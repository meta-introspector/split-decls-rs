macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! ConstChoice {
    () => {
        deps!();
        # [doc = " A boolean value returned by constant-time `const fn`s."] # [derive (Debug , Copy , Clone)] pub struct ConstChoice (Word) ;
    };
}

ConstChoice!();
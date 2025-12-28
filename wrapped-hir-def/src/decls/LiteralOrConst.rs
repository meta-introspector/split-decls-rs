macro_rules! deps {
    () => {
        Const!();
        Literal!();
        PatId!();
    };
}

macro_rules! LiteralOrConst {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] # [doc = " Used in range patterns."] pub enum LiteralOrConst { Literal (Literal) , Const (PatId) , }
    };
}

LiteralOrConst!();
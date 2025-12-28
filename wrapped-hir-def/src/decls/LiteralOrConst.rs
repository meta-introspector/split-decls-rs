macro_rules! deps {
    () => {
        PatId!();
        Const!();
        Literal!();
    };
}

macro_rules! LiteralOrConst {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] # [doc = " Used in range patterns."] pub enum LiteralOrConst { Literal (Literal) , Const (PatId) , }
    };
}

LiteralOrConst!()
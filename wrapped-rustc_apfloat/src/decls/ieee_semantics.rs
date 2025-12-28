macro_rules! deps {
    () => {
        Semantics!();
        IeeeFloat!();
    };
}

macro_rules! ieee_semantics {
    () => {
        deps!();
        macro_rules ! ieee_semantics { ($ ($ (# [$ meta : meta]) * $ name : ident = $ sem : ident ($ bits : tt : $ exp_bits : tt) $ ({ $ ($ extra : tt) * }) ?) ,* $ (,) ?) => { $ (# [doc = concat ! ("Floating point semantics for [`" , stringify ! ($ name) , "`].")] # [doc = ""] # [doc = " See that type for more details."] pub struct $ sem ; $ (# [$ meta]) * pub type $ name = IeeeFloat <$ sem >; impl Semantics for $ sem { const BITS : usize = $ bits ; const EXP_BITS : usize = $ exp_bits ; $ ($ ($ extra) *) ? }) * } }
    };
}

ieee_semantics!()
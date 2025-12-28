macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! partialeq_numeric {
    () => {
        deps!();
        macro_rules ! partialeq_numeric { ($ ($ eq : ident [$ ($ ty : ty) *]) *) => { $ ($ (impl PartialEq <$ ty > for Value { fn eq (& self , other : &$ ty) -> bool { $ eq (self , * other as _) } } impl PartialEq < Value > for $ ty { fn eq (& self , other : & Value) -> bool { $ eq (other , * self as _) } } impl <'a > PartialEq <$ ty > for &'a Value { fn eq (& self , other : &$ ty) -> bool { $ eq (* self , * other as _) } } impl <'a > PartialEq <$ ty > for &'a mut Value { fn eq (& self , other : &$ ty) -> bool { $ eq (* self , * other as _) } }) *) * } }
    };
}

partialeq_numeric!();
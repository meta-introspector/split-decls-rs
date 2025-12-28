macro_rules! deps {
    () => {
        GePrecomp!();
        GeP1P1!();
        GeP3!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl Sub < GePrecomp > for GeP3 { type Output = GeP1P1 ; fn sub (self , _rhs : GePrecomp) -> GeP1P1 { let y1_plus_x1 = self . y + self . x ; let y1_minus_x1 = self . y - self . x ; let a = y1_plus_x1 * _rhs . y_minus_x ; let b = y1_minus_x1 * _rhs . y_plus_x ; let c = _rhs . xy2d * self . t ; let d = self . z + self . z ; let x3 = a - b ; let y3 = a + b ; let z3 = d - c ; let t3 = d + c ; GeP1P1 { x : x3 , y : y3 , z : z3 , t : t3 , } } }
    };
}

impl_113!()
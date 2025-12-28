macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! Or {
    () => {
        deps!();
        # [doc = " Guard for [`GuardExt::or`](trait.GuardExt.html#method.or)."] pub struct Or < A : Guard , B : Guard > (A , B) ;
    };
}

Or!()
macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! And {
    () => {
        deps!();
        # [doc = " Guard for [`GuardExt::and`](trait.GuardExt.html#method.and)."] pub struct And < A : Guard , B : Guard > (A , B) ;
    };
}

And!()
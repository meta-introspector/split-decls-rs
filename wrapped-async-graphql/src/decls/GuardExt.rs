macro_rules! deps {
    () => {
        Guard!();
        Or!();
        And!();
    };
}

macro_rules! GuardExt {
    () => {
        deps!();
        # [doc = " An extension trait for `Guard`."] pub trait GuardExt : Guard + Sized { # [doc = " Perform `and` operator on two rules"] fn and < R : Guard > (self , other : R) -> And < Self , R > { And (self , other) } # [doc = " Perform `or` operator on two rules"] fn or < R : Guard > (self , other : R) -> Or < Self , R > { Or (self , other) } }
    };
}

GuardExt!()
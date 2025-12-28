macro_rules! GroupOps {
    () => {
        # [doc = " A helper trait for types with a group operation."] pub trait GroupOps < Rhs = Self , Output = Self > : Add < Rhs , Output = Output > + Sub < Rhs , Output = Output > + AddAssign < Rhs > + SubAssign < Rhs > { }
    };
}

GroupOps!();
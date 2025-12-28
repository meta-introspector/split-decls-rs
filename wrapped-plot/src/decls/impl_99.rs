macro_rules! deps {
    () => {
        Boxed!();
        Properties!();
        Set!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Set < Boxed > for Properties { # [doc = " Select if the key will be surrounded with a box or not"] # [doc = ""] # [doc = " **Note** The key is not boxed by default"] fn set (& mut self , boxed : Boxed) -> & mut Properties { match boxed { Boxed :: No => self . boxed = false , Boxed :: Yes => self . boxed = true , } self } }
    };
}

impl_99!();
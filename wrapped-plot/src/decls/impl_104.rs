macro_rules! deps {
    () => {
        Title!();
        Properties!();
        Set!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Set < Title > for Properties { fn set (& mut self , title : Title) -> & mut Properties { self . title = Some (title . 0) ; self } }
    };
}

impl_104!();
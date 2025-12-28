macro_rules! deps {
    () => {
        Properties!();
        Set!();
        Title!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Set < Title > for Properties { fn set (& mut self , title : Title) -> & mut Properties { self . title = Some (title . 0) ; self } }
    };
}

impl_104!()
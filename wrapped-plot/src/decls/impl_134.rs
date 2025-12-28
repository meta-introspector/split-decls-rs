macro_rules! deps {
    () => {
        Set!();
        Figure!();
        Title!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl Set < Title > for Figure { # [doc = " Sets the title"] fn set (& mut self , title : Title) -> & mut Figure { self . title = Some (title . 0) ; self } }
    };
}

impl_134!();
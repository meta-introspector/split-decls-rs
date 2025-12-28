macro_rules! deps {
    () => {
        Properties!();
        Set!();
        LineWidth!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Set < LineWidth > for Properties { # [doc = " Changes the linewidth"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `lw` is a non-positive value"] fn set (& mut self , lw : LineWidth) -> & mut Properties { let lw = lw . 0 ; assert ! (lw > 0.) ; self . linewidth = Some (lw) ; self } }
    };
}

impl_71!();
macro_rules! deps {
    () => {
        Set!();
        LineWidth!();
        Properties!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Set < LineWidth > for Properties { # [doc = " Changes the width of the line"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `width` is a non-positive value"] fn set (& mut self , lw : LineWidth) -> & mut Properties { let lw = lw . 0 ; assert ! (lw > 0.) ; self . linewidth = Some (lw) ; self } }
    };
}

impl_45!()
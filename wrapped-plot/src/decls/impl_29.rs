macro_rules! deps {
    () => {
        Properties!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Properties { # [doc = " Hides the axis"] # [doc = ""] # [doc = " **Note** The `TopX` and `RightY` axes are hidden by default"] pub fn hide (& mut self) -> & mut Properties { self . hidden = true ; self } # [doc = " Makes the axis visible"] # [doc = ""] # [doc = " **Note** The `BottomX` and `LeftY` axes are visible by default"] pub fn show (& mut self) -> & mut Properties { self . hidden = false ; self } }
    };
}

impl_29!();
macro_rules! deps {
    () => {
        Properties!();
        Justification!();
        Set!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Set < Justification > for Properties { # [doc = " Changes the justification of the text of each entry"] # [doc = ""] # [doc = " **Note** The text is `RightJustified` by default"] fn set (& mut self , justification : Justification) -> & mut Properties { self . justification = Some (justification) ; self } }
    };
}

impl_100!()
macro_rules! deps {
    () => {
        Figure!();
        Set!();
        Font!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Set < Font > for Figure { # [doc = " Changes the font"] fn set (& mut self , font : Font) -> & mut Figure { self . font = Some (font . 0) ; self } }
    };
}

impl_129!();
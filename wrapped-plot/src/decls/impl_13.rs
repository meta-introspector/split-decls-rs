macro_rules! deps {
    () => {
        Color!();
        Display!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Display < Cow < 'static , str > > for Color { fn display (& self) -> Cow < 'static , str > { match * self { Color :: Black => Cow :: from ("black") , Color :: Blue => Cow :: from ("blue") , Color :: Cyan => Cow :: from ("cyan") , Color :: DarkViolet => Cow :: from ("dark-violet") , Color :: ForestGreen => Cow :: from ("forest-green") , Color :: Gold => Cow :: from ("gold") , Color :: Gray => Cow :: from ("gray") , Color :: Green => Cow :: from ("green") , Color :: Magenta => Cow :: from ("magenta") , Color :: Red => Cow :: from ("red") , Color :: Rgb (r , g , b) => Cow :: from (format ! ("#{:02x}{:02x}{:02x}" , r , g , b)) , Color :: White => Cow :: from ("white") , Color :: Yellow => Cow :: from ("yellow") , } } }
    };
}

impl_13!();
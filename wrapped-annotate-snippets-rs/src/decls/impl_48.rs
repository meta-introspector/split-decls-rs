macro_rules! deps {
    () => {
        ElementStyle!();
        Stylesheet!();
        Level!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl ElementStyle { pub (crate) fn color_spec (& self , level : & Level < '_ > , stylesheet : & Stylesheet) -> Style { match self { ElementStyle :: Addition => stylesheet . addition , ElementStyle :: Removal => stylesheet . removal , ElementStyle :: LineAndColumn => stylesheet . none , ElementStyle :: LineNumber => stylesheet . line_num , ElementStyle :: Quotation => stylesheet . none , ElementStyle :: MainHeaderMsg => stylesheet . emphasis , ElementStyle :: UnderlinePrimary | ElementStyle :: LabelPrimary => level . style (stylesheet) , ElementStyle :: UnderlineSecondary | ElementStyle :: LabelSecondary => stylesheet . context , ElementStyle :: HeaderMsg | ElementStyle :: NoStyle => stylesheet . none , ElementStyle :: Level (lvl) => lvl . style (stylesheet) , } } }
    };
}

impl_48!()
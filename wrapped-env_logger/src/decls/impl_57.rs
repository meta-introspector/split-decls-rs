macro_rules! deps {
    () => {
        WriteStyle!();
        StyledValue!();
        DefaultVisitSource!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl DefaultVisitSource < '_ > { fn style_key < 'k > (& self , text : Key < 'k >) -> StyledValue < Key < 'k > > { # [cfg (feature = "color")] { StyledValue { style : if self . 0 . write_style == WriteStyle :: Never { Style :: new () } else { Style :: new () . italic () } , value : text , } } # [cfg (not (feature = "color"))] { text } } }
    };
}

impl_57!();
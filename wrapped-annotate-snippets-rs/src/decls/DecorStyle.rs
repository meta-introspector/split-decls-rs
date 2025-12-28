macro_rules! DecorStyle {
    () => {
        # [doc = " The character set for rendering for decor"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum DecorStyle { Ascii , Unicode , }
    };
}

DecorStyle!();
macro_rules! RenderOption {
    () => {
        # [derive (Clone , PartialEq , Eq , Debug)] pub enum RenderOption { NoEdgeLabels , NoNodeLabels , NoEdgeStyles , NoNodeStyles , Fontname (String) , DarkTheme , }
    };
}

RenderOption!();
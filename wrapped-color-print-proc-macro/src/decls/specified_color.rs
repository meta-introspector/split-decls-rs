macro_rules! deps {
    () => {
        Input!();
        Color!();
        Specified!();
        Color256!();
        ColorRgb!();
        Case!();
        Color16!();
        Result!();
    };
}

macro_rules! specified_color {
    () => {
        deps!();
        # [doc = " Parses a color which has been prefixed by a specifier like `\"bg:\"` or `\"fg:\"`."] fn specified_color (input : Input < '_ >) -> Result < '_ , Color > { with_failure_message (alt ((map (color_16 (Case :: Lowercase) , Color :: Color16) , map (color_256 (Specified :: True) , | (color , _) | Color :: Color256 (color)) , map (color_rgb (Specified :: True) , | (color , _) | Color :: ColorRgb (color)) ,)) , "Unknown color") (input) }
    };
}

specified_color!()
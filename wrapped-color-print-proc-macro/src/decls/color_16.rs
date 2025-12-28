macro_rules! deps {
    () => {
        Intensity!();
        Parser!();
        Case!();
        Color16!();
    };
}

macro_rules! color_16 {
    () => {
        deps!();
        # [doc = " Parses a basic color like `\"blue\"`, `\"b\"`, `\"blue!\"`, `\"bright-blue\"`, with the given letter"] # [doc = " case."] fn color_16 < 'a > (letter_case : Case) -> impl Parser < 'a , Color16 > { move | input | { let bright_prefix = match letter_case { Case :: Uppercase => "BRIGHT-" , Case :: Lowercase => "bright-" , } ; alt ((map (preceded (tag (bright_prefix) , base_color (letter_case)) , | base_color | Color16 :: new (base_color , Intensity :: Bright)) , map (pair (spaced (base_color (letter_case)) , is_present (spaced (tag ("!")))) , | (base_color , is_bright) | Color16 :: new (base_color , Intensity :: new (is_bright))))) (input) } }
    };
}

color_16!();
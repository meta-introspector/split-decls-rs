macro_rules! deps {
    () => {
        ColorKind!();
        ColorRgb!();
        Specified!();
        Input!();
        Result!();
        Parser!();
    };
}

macro_rules! color_rgb {
    () => {
        deps!();
        # [doc = " Parses a true-color color, like `\"rgb(10,20,30)\"`. If the color to parse is declared as"] # [doc = " \"specified\", the only the lowercase functions will be available."] fn color_rgb < 'a > (specified : Specified) -> impl Parser < 'a , (ColorRgb , Option < ColorKind >) > { fn component (input : Input < '_ >) -> Result < '_ , u8 > { with_failure_message (u8 , "Bad RGB color component: must be a number between 0 and 255") (input) } fn rgb_fn (name : & str) -> impl Parser < '_ , ColorRgb > { map (function (tag (name) , with_failure_message (tuple ((component , stag (",") , component , stag (",") , component)) , "Wrong arguments: expects 3 numbers between 0 and 255, separated by commas")) , | (r , _ , g , _ , b) | ColorRgb { r , g , b }) } fn rgb_lower (input : Input < '_ >) -> Result < '_ , ColorRgb > { rgb_fn ("rgb") (input) } fn rgb_upper (input : Input < '_ >) -> Result < '_ , ColorRgb > { rgb_fn ("RGB") (input) } if specified . is_true () { | input | { map (alt ((rgb_lower , hex_rgb_color)) , | color | (color , None)) (input) } } else { | input | { alt ((map (rgb_lower , | color | (color , Some (ColorKind :: Foreground))) , map (rgb_upper , | color | (color , Some (ColorKind :: Background))) , map (hex_rgb_color , | color | (color , Some (ColorKind :: Foreground))) ,)) (input) } } }
    };
}

color_rgb!()
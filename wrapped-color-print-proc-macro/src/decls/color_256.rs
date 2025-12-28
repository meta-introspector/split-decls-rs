macro_rules! deps {
    () => {
        Result!();
        Parser!();
        Specified!();
        Color256!();
        Input!();
        ColorKind!();
    };
}

macro_rules! color_256 {
    () => {
        deps!();
        # [doc = " Parses a 256-color color, like `\"pal(42)\"`. If the color to parse is declared as \"specified\","] # [doc = " the only the lowercase functions will be available."] fn color_256 < 'a > (specified : Specified) -> impl Parser < 'a , (Color256 , Option < ColorKind >) > { const PALETTE_FAILURE_MESSAGE : & str = "Palette color must a number between 0 and 255" ; fn pal_color (input : Input < '_ >) -> Result < '_ , u8 > { with_failure_message (u8 , PALETTE_FAILURE_MESSAGE) (input) } fn pal_fn < 'a > (name1 : & 'a str , name2 : & 'a str , name3 : & 'a str) -> impl Parser < 'a , u8 > { let function_names = alt ((tag (name1) , tag (name2) , tag (name3))) ; function (function_names , with_failure_message (pal_color , PALETTE_FAILURE_MESSAGE)) } fn pal_lower (input : Input < '_ >) -> Result < '_ , Color256 > { map (alt ((pal_fn ("palette" , "pal" , "p") , check_parser_before_failure (digit1 , u8 , PALETTE_FAILURE_MESSAGE))) , Color256) (input) } fn pal_upper (input : Input < '_ >) -> Result < '_ , Color256 > { map (pal_fn ("PALETTE" , "PAL" , "P") , Color256) (input) } if specified . is_true () { | input | { map (pal_lower , | color | (color , None)) (input) } } else { | input | { alt ((map (pal_lower , | color | (color , Some (ColorKind :: Foreground))) , map (pal_upper , | color | (color , Some (ColorKind :: Background))))) (input) } } }
    };
}

color_256!();
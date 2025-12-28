macro_rules! deps {
    () => {
        ColorRgb!();
        Input!();
        Result!();
    };
}

macro_rules! hex_rgb_color {
    () => {
        deps!();
        # [doc = " Parses an HTML-like color like `\"#aabbcc\"`."] fn hex_rgb_color (input : Input < '_ >) -> Result < '_ , ColorRgb > { fn component (input : Input < '_ >) -> Result < '_ , u8 > { map_res (take_while_m_n (2 , 2 , | c : char | c . is_ascii_hexdigit ()) , | input | u8 :: from_str_radix (input , 16)) (input) } map (preceded (tag ("#") , with_failure_message (tuple ((component , component , component)) , "Bad hexadecimal color code")) , | (r , g , b) | ColorRgb { r , g , b }) (input) }
    };
}

hex_rgb_color!();
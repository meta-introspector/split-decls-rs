macro_rules! deps {
    () => {
        Error!();
        Name!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl FromStr for Name { type Err = Error ; fn from_str (mut s : & str) -> Result < Self , Self :: Err > { let bright = if let Some (rest) = s . strip_prefix ("bright") { s = rest ; true } else { false } ; match s { "normal" if ! bright => return Ok (Self :: Normal) , "-1" if ! bright => return Ok (Self :: Normal) , "normal" if bright => return Err (color_err (s)) , "default" if ! bright => return Ok (Self :: Default) , "default" if bright => return Err (color_err (s)) , "black" if ! bright => return Ok (Self :: Black) , "black" if bright => return Ok (Self :: BrightBlack) , "red" if ! bright => return Ok (Self :: Red) , "red" if bright => return Ok (Self :: BrightRed) , "green" if ! bright => return Ok (Self :: Green) , "green" if bright => return Ok (Self :: BrightGreen) , "yellow" if ! bright => return Ok (Self :: Yellow) , "yellow" if bright => return Ok (Self :: BrightYellow) , "blue" if ! bright => return Ok (Self :: Blue) , "blue" if bright => return Ok (Self :: BrightBlue) , "magenta" if ! bright => return Ok (Self :: Magenta) , "magenta" if bright => return Ok (Self :: BrightMagenta) , "cyan" if ! bright => return Ok (Self :: Cyan) , "cyan" if bright => return Ok (Self :: BrightCyan) , "white" if ! bright => return Ok (Self :: White) , "white" if bright => return Ok (Self :: BrightWhite) , _ => () , } if let Ok (v) = u8 :: from_str (s) { return Ok (Self :: Ansi (v)) ; } if let Some (s) = s . strip_prefix ('#') { if s . len () == 6 && s . is_char_boundary (2) && s . is_char_boundary (4) && s . is_char_boundary (6) { let rgb = (u8 :: from_str_radix (& s [.. 2] , 16) , u8 :: from_str_radix (& s [2 .. 4] , 16) , u8 :: from_str_radix (& s [4 ..] , 16) ,) ; if let (Ok (r) , Ok (g) , Ok (b)) = rgb { return Ok (Self :: Rgb (r , g , b)) ; } } } Err (color_err (s)) } }
    };
}

impl_20!();
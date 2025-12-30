// Generated macro for parse_color (function)
macro_rules! Depcrateparse_color {
() => {
// Module: crate
// Provides: {"parse_color"}
// Dependencies: {}
fn parse_color (word : & str) -> Result < Option < anstyle :: Color > , () > { let color = match word { "normal" => None , "-1" => None , "black" => Some (anstyle :: AnsiColor :: Black . into ()) , "red" => Some (anstyle :: AnsiColor :: Red . into ()) , "green" => Some (anstyle :: AnsiColor :: Green . into ()) , "yellow" => Some (anstyle :: AnsiColor :: Yellow . into ()) , "blue" => Some (anstyle :: AnsiColor :: Blue . into ()) , "magenta" => Some (anstyle :: AnsiColor :: Magenta . into ()) , "cyan" => Some (anstyle :: AnsiColor :: Cyan . into ()) , "white" => Some (anstyle :: AnsiColor :: White . into ()) , _ => { if let Some (hex) = word . strip_prefix ('#') { let l = hex . len () ; if l != 3 && l != 6 { return Err (()) ; } let l = l / 3 ; if let (Ok (r) , Ok (g) , Ok (b)) = (u8 :: from_str_radix (& hex [0 .. l] , 16) , u8 :: from_str_radix (& hex [l .. (2 * l)] , 16) , u8 :: from_str_radix (& hex [(2 * l) .. (3 * l)] , 16) ,) { Some (anstyle :: Color :: from ((r , g , b))) } else { return Err (()) ; } } else if let Ok (n) = word . parse :: < u8 > () { Some (anstyle :: Color :: from (n)) } else { return Err (()) ; } } } ; Ok (color) }
};
}

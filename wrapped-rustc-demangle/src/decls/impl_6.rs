macro_rules! deps {
    () => {
        Demangle!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for Demangle < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut inner = self . inner ; for element in 0 .. self . elements { let mut rest = inner ; while rest . chars () . next () . unwrap () . is_digit (10) { rest = & rest [1 ..] ; } let i : usize = inner [.. (inner . len () - rest . len ())] . parse () . unwrap () ; inner = & rest [i ..] ; rest = & rest [.. i] ; if f . alternate () && element + 1 == self . elements && is_rust_hash (& rest) { break ; } if element != 0 { f . write_str ("::") ? ; } if rest . starts_with ("_$") { rest = & rest [1 ..] ; } loop { if rest . starts_with ('.') { if let Some ('.') = rest [1 ..] . chars () . next () { f . write_str ("::") ? ; rest = & rest [2 ..] ; } else { f . write_str (".") ? ; rest = & rest [1 ..] ; } } else if rest . starts_with ('$') { let (escape , after_escape) = if let Some (end) = rest [1 ..] . find ('$') { (& rest [1 ..= end] , & rest [end + 2 ..]) } else { break ; } ; let unescaped = match escape { "SP" => "@" , "BP" => "*" , "RF" => "&" , "LT" => "<" , "GT" => ">" , "LP" => "(" , "RP" => ")" , "C" => "," , _ => { if escape . starts_with ('u') { let digits = & escape [1 ..] ; let all_lower_hex = digits . chars () . all (| c | match c { '0' ..= '9' | 'a' ..= 'f' => true , _ => false , }) ; let c = u32 :: from_str_radix (digits , 16) . ok () . and_then (char :: from_u32) ; if let (true , Some (c)) = (all_lower_hex , c) { if ! c . is_control () { c . fmt (f) ? ; rest = after_escape ; continue ; } } } break ; } } ; f . write_str (unescaped) ? ; rest = after_escape ; } else if let Some (i) = rest . find (| c | c == '$' || c == '.') { f . write_str (& rest [.. i]) ? ; rest = & rest [i ..] ; } else { break ; } } f . write_str (rest) ? ; } Ok (()) } }
    };
}

impl_6!();
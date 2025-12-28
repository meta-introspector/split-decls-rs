macro_rules! string_value {
    () => {
        pub (super) fn string_value (s : & str) -> String { let mut chars = s . chars () ; std :: iter :: from_fn (| | { Some (match chars . next () ? { '\\' => match chars . next () . expect ("backslash at end") { c @ '\"' | c @ '\\' | c @ '/' => c , 'b' => '\x08' , 'f' => '\x0C' , 'n' => '\n' , 'r' => '\r' , 't' => '\t' , 'u' => std :: char :: from_u32 ((0 .. 4) . map (| _ | chars . next () . unwrap () . to_digit (16) . unwrap ()) . fold (0 , | acc , digit | acc * 16 + digit) ,) . unwrap () , _ => unreachable ! () , } , other => other , }) }) . collect () }
    };
}

string_value!();
macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! hex_check_fallback_with_case {
    () => {
        deps!();
        # [doc = " Check if the input is valid hex bytes slice with case check"] pub fn hex_check_fallback_with_case (src : & [u8] , check_case : CheckCase) -> bool { match check_case { CheckCase :: None => src . iter () . all (| & x | UNHEX [x as usize] != NIL) , CheckCase :: Lower => src . iter () . all (| & x | UNHEX_LOWER [x as usize] != NIL) , CheckCase :: Upper => src . iter () . all (| & x | UNHEX_UPPER [x as usize] != NIL) , } }
    };
}

hex_check_fallback_with_case!()
macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! init_unhex_array {
    () => {
        deps!();
        const fn init_unhex_array (check_case : CheckCase) -> [u8 ; 256] { let mut arr = [0 ; 256] ; let mut i = 0 ; while i < 256 { arr [i] = match i as u8 { b'0' ..= b'9' => i as u8 - b'0' , b'a' ..= b'f' => match check_case { CheckCase :: Lower | CheckCase :: None => i as u8 - b'a' + 10 , _ => NIL , } , b'A' ..= b'F' => match check_case { CheckCase :: Upper | CheckCase :: None => i as u8 - b'A' + 10 , _ => NIL , } , _ => NIL , } ; i += 1 ; } arr }
    };
}

init_unhex_array!()
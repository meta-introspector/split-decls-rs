macro_rules! value_to_digit {
    () => {
        # [inline] fn value_to_digit (value : u32) -> char { match value { 0 ..= 25 => (value as u8 + b'a') as char , 26 ..= 35 => (value as u8 - 26 + b'0') as char , _ => panic ! () , } }
    };
}

value_to_digit!();
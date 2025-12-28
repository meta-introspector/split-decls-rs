macro_rules! span_for_digits {
    () => {
        const fn span_for_digits (digits : u16) -> u32 { match digits { 0 => 1_000_000_000 , 1 => 100_000_000 , 2 => 10_000_000 , 3 => 1_000_000 , 4 => 100_000 , 5 => 10_000 , 6 => 1_000 , 7 => 100 , 8 => 10 , _ => 1 , } }
    };
}

span_for_digits!()
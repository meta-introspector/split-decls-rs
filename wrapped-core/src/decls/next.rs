macro_rules! next {
    () => {
        fn next (bytes : & mut core :: str :: Bytes , len : usize , delimiter : bool) -> Option < u32 > { let mut value : u32 = 0 ; for _ in 0 .. len { let digit = bytes . next () ? ; match digit { b'0' ..= b'9' => value = (value << 4) + (digit - b'0') as u32 , b'A' ..= b'F' => value = (value << 4) + (digit - b'A' + 10) as u32 , b'a' ..= b'f' => value = (value << 4) + (digit - b'a' + 10) as u32 , _ => return None , } } if delimiter && bytes . next () != Some (b'-') { None } else { Some (value) } }
    };
}

next!()
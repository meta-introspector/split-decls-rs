macro_rules! read_token {
    () => {
        fn read_token (input : & str , token : u8) -> & str { for (pos , c) in input . bytes () . enumerate () { if c == token { return & input [pos + 1 ..] ; } else if c != b' ' && c != b',' { break ; } } panic ! ("`{}` expected" , token . escape_ascii ()) ; }
    };
}

read_token!()
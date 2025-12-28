macro_rules! deps {
    () => {
        AsyncReadExt!();
        Repeat!();
    };
}

macro_rules! repeat {
    () => {
        deps!();
        # [doc = " Creates an infinite reader that reads the same byte repeatedly."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{self, AsyncReadExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut reader = io::repeat(b'a');"] # [doc = ""] # [doc = " let mut contents = vec![0; 5];"] # [doc = " reader.read_exact(&mut contents).await?;"] # [doc = " assert_eq!(contents, b\"aaaaa\");"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn repeat (byte : u8) -> Repeat { Repeat { byte } }
    };
}

repeat!()
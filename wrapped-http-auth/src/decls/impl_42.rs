macro_rules! deps {
    () => {
        Algorithm!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Algorithm { # [doc = " Parses a string into a tuple of `Algorithm` and a bool representing"] # [doc = " whether the `-sess` suffix is present."] fn parse (s : & str) -> Result < (Self , bool) , String > { Ok (match s { "MD5" => (Algorithm :: Md5 , false) , "MD5-sess" => (Algorithm :: Md5 , true) , "SHA-256" => (Algorithm :: Sha256 , false) , "SHA-256-sess" => (Algorithm :: Sha256 , true) , "SHA-512-256" => (Algorithm :: Sha512Trunc256 , false) , "SHA-512-256-sess" => (Algorithm :: Sha512Trunc256 , true) , _ => return Err (format ! ("unknown algorithm {:?}" , s)) , }) } # [inline (never)] fn as_str (& self , session : bool) -> & 'static str { match (self , session) { (Algorithm :: Md5 , false) => "MD5" , (Algorithm :: Md5 , true) => "MD5-sess" , (Algorithm :: Sha256 , false) => "SHA-256" , (Algorithm :: Sha256 , true) => "SHA-256-sess" , (Algorithm :: Sha512Trunc256 , false) => "SHA-512-256" , (Algorithm :: Sha512Trunc256 , true) => "SHA-512-256-sess" , } } # [inline (never)] fn h (& self , items : & [& [u8]]) -> String { match self { Algorithm :: Md5 => h (md5 :: Md5 :: new () , items) , Algorithm :: Sha256 => h (sha2 :: Sha256 :: new () , items) , Algorithm :: Sha512Trunc256 => h (sha2 :: Sha512_256 :: new () , items) , } } }
    };
}

impl_42!()
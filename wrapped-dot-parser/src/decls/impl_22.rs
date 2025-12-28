macro_rules! deps {
    () => {
        Graphs!();
        GraphFromFileError!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Graphs < (String , String) > { # [allow (clippy :: result_large_err)] # [doc = " Parses multiple graphs from a file, which path is given. Notice that when doing so, attributes are"] # [doc = " of type `(String, String)`, not the default `(&'a str, &'a str)`, since we are reading and taking"] # [doc = " ownership of the content of the file."] pub fn from_file < 'a , P > (p : P) -> Result < Self , GraphFromFileError < 'a > > where P : AsRef < Path > , { let s = std :: fs :: read_to_string (p) ? ; let mut pairs = DotParser :: parse (Rule :: dotfile , & s) ? ; let pair = pairs . next () . expect ("The toplevel `Pairs` is empty.") ; match Graphs :: try_from (pair) { Ok (g) => Ok (g) , Err (e) => panic ! ("{}" , e) , } } }
    };
}

impl_22!();
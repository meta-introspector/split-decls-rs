macro_rules! deps {
    () => {
        Scheme!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for Scheme { fn from (value : & 'a str) -> Self { match value { "ssh" | "ssh+git" | "git+ssh" => Scheme :: Ssh , "file" => Scheme :: File , "git" => Scheme :: Git , "http" => Scheme :: Http , "https" => Scheme :: Https , unknown => Scheme :: Ext (unknown . into ()) , } } }
    };
}

impl_9!();
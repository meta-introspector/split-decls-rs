macro_rules! deps {
    () => {
        Scheme!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Scheme { # [doc = " Return ourselves parseable name."] pub fn as_str (& self) -> & str { use Scheme :: * ; match self { File => "file" , Git => "git" , Ssh => "ssh" , Http => "http" , Https => "https" , Ext (name) => name . as_str () , } } # [doc = " Return the default port for this scheme, or `None` if it is not known."] pub fn default_port (& self) -> Option < u16 > { match self { Scheme :: Http => Some (80) , Scheme :: Https => Some (443) , Scheme :: Ssh => Some (22) , Scheme :: Git => Some (9418) , _ => None , } } }
    };
}

impl_10!();
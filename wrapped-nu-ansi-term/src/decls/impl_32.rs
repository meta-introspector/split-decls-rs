macro_rules! deps {
    () => {
        AnsiGenericString!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [doc = " Cloning an `AnsiGenericString` will clone its underlying string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::AnsiString;"] # [doc = ""] # [doc = " let plain_string = AnsiString::from(\"a plain string\");"] # [doc = " let clone_string = plain_string.clone();"] # [doc = " assert_eq!(clone_string, plain_string);"] # [doc = " ```"] impl < 'a , S : 'a + ToOwned + ? Sized > Clone for AnsiGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , { fn clone (& self) -> AnsiGenericString < 'a , S > { AnsiGenericString { style : self . style , string : self . string . clone () , oscontrol : self . oscontrol . clone () , } } }
    };
}

impl_32!();
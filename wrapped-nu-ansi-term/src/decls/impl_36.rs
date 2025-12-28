macro_rules! deps {
    () => {
        Style!();
        AnsiGenericString!();
        OSControl!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a , S : 'a + ToOwned + ? Sized > AnsiGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , { # [doc = " Directly access the style"] pub const fn style_ref (& self) -> & Style { & self . style } # [doc = " Directly access the style mutably"] pub fn style_ref_mut (& mut self) -> & mut Style { & mut self . style } # [doc = " Directly access the underlying string"] pub fn as_str (& self) -> & S { self . string . as_ref () } # [doc = " Produce an ANSI string that changes the title shown"] # [doc = " by the terminal emulator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::AnsiGenericString;"] # [doc = " let title_string = AnsiGenericString::title(\"My Title\");"] # [doc = " println!(\"{}\", title_string);"] # [doc = " ```"] # [doc = " Should produce an empty line but set the terminal title."] pub fn title < I > (s : I) -> Self where I : Into < Cow < 'a , S > > , { Self { style : Style :: default () , string : s . into () , oscontrol : Some (OSControl :: < 'a , S > :: Title) , } } # [doc = " Cause the styled ANSI string to link to the given URL"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::Color::Red;"] # [doc = ""] # [doc = " let link_string = Red.paint(\"a red string\").hyperlink(\"https://www.example.com\");"] # [doc = " println!(\"{}\", link_string);"] # [doc = " ```"] # [doc = " Should show a red-painted string which, on terminals"] # [doc = " that support it, is a clickable hyperlink."] pub fn hyperlink < I > (mut self , url : I) -> Self where I : Into < Cow < 'a , S > > , { self . oscontrol = Some (OSControl :: Link { url : url . into () }) ; self } # [doc = " Get any URL associated with the string"] pub fn url_string (& self) -> Option < & S > { match & self . oscontrol { Some (OSControl :: Link { url : u }) => Some (u . as_ref ()) , _ => None , } } }
    };
}

impl_36!()
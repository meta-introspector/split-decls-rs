macro_rules! deps {
    () => {
        StyledStr!();
        PossibleValue!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        # [doc = " Reflection"] impl PossibleValue { # [doc = " Get the name of the argument value"] # [inline] pub fn get_name (& self) -> & str { self . name . as_str () } # [doc = " Get the help specified for this argument, if any"] # [inline] pub fn get_help (& self) -> Option < & StyledStr > { self . help . as_ref () } # [doc = " Report if [`PossibleValue::hide`] is set"] # [inline] pub fn is_hide_set (& self) -> bool { self . hide } # [doc = " Report if `PossibleValue` is not hidden and has a help message"] pub (crate) fn should_show_help (& self) -> bool { ! self . hide && self . help . is_some () } # [doc = " Get the name if argument value is not hidden, `None` otherwise,"] # [doc = " but wrapped in quotes if it contains whitespace"] # [cfg (feature = "help")] pub (crate) fn get_visible_quoted_name (& self) -> Option < std :: borrow :: Cow < '_ , str > > { if ! self . hide { Some (if self . name . contains (char :: is_whitespace) { format ! ("{:?}" , self . name) . into () } else { self . name . as_str () . into () }) } else { None } } # [doc = " Returns all valid values of the argument value."] # [doc = ""] # [doc = " Namely the name and all aliases."] pub fn get_name_and_aliases (& self) -> impl Iterator < Item = & str > + '_ { std :: iter :: once (self . get_name ()) . chain (self . aliases . iter () . map (| s | s . as_str ())) } # [doc = " Tests if the value is valid for this argument value"] # [doc = ""] # [doc = " The value is valid if it is either the name or one of the aliases."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::builder::PossibleValue;"] # [doc = " let arg_value = PossibleValue::new(\"fast\").alias(\"not-slow\");"] # [doc = ""] # [doc = " assert!(arg_value.matches(\"fast\", false));"] # [doc = " assert!(arg_value.matches(\"not-slow\", false));"] # [doc = ""] # [doc = " assert!(arg_value.matches(\"FAST\", true));"] # [doc = " assert!(!arg_value.matches(\"FAST\", false));"] # [doc = " ```"] pub fn matches (& self , value : & str , ignore_case : bool) -> bool { if ignore_case { self . get_name_and_aliases () . any (| name | eq_ignore_case (name , value)) } else { self . get_name_and_aliases () . any (| name | name == value) } } }
    };
}

impl_148!()
macro_rules! deps {
    () => {
        AttrValue!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'string > AttrValue < 'string > { # [doc = " Returns the state of an attribute by inspecting its [value](crate::Repository::get_attr)"] # [doc = " by a [string](prim@str)."] # [doc = ""] # [doc = " This function always returns [`AttrValue::String`] and never returns [`AttrValue::Bytes`]"] # [doc = " when the attribute is set to a string."] pub fn from_string (value : Option < & 'string str >) -> Self { from_value ! (value => Self :: String (value . unwrap ())) } # [doc = " Returns the state of an attribute by inspecting its [value](crate::Repository::get_attr_bytes)"] # [doc = " by a [byte](u8) [slice]."] # [doc = ""] # [doc = " This function will perform UTF-8 validation when the attribute is set to a string, returns"] # [doc = " [`AttrValue::String`] if it's valid UTF-8 and [`AttrValue::Bytes`] otherwise."] pub fn from_bytes (value : Option < & 'string [u8] >) -> Self { let mut value = Self :: always_bytes (value) ; if let Self :: Bytes (bytes) = value { if let Ok (string) = str :: from_utf8 (bytes) { value = Self :: String (string) ; } } value } # [doc = " Returns the state of an attribute just like [`AttrValue::from_bytes`], but skips UTF-8"] # [doc = " validation and always returns [`AttrValue::Bytes`] when it's set to a string."] pub fn always_bytes (value : Option < & 'string [u8] >) -> Self { from_value ! (value => Self :: Bytes (value . unwrap ())) } }
    };
}

impl_48!()
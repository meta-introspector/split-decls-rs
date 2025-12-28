macro_rules! deps {
    () => {
        Boolean!();
        Binary!();
    };
}

macro_rules! binary {
    () => {
        deps!();
        mod binary { use crate :: config :: tree :: diff :: Binary ; impl Binary { # [doc = " Convert `value` into a tri-state boolean that can take the special value `auto`, resulting in `None`, or is a boolean."] # [doc = " If `None` is given, it's treated as implicit boolean `true`, as this method is made to be used"] # [doc = " with [`gix_config::file::section::Body::value_implicit()`]."] pub fn try_into_binary (& 'static self , value : Option < std :: borrow :: Cow < '_ , crate :: bstr :: BStr > > ,) -> Result < Option < bool > , crate :: config :: key :: GenericErrorWithValue > { Ok (match value { None => Some (true) , Some (value) => { if value . as_ref () == "auto" { None } else { Some (gix_config :: Boolean :: try_from (value . as_ref ()) . map (| b | b . 0) . map_err (| err | { crate :: config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ()) . with_source (err) }) ? ,) } } }) } } }
    };
}

binary!()
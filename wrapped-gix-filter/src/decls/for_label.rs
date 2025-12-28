macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! for_label {
    () => {
        deps!();
        # [doc = " Try to produce a new `Encoding` for `label` or report an error if it is not known."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " * There is no special handling of UTF-16LE/BE with checks if data contains a BOM or not, like `git` as we don't expect to have"] # [doc = "   data available here."] # [doc = " * Special `-BOM` suffixed versions of `UTF-16` encodings are not supported."] pub fn for_label < 'a > (label : impl Into < & 'a BStr >) -> Result < & 'static Encoding , for_label :: Error > { let mut label = label . into () ; if label == "latin-1" { label = "ISO-8859-1" . into () ; } let enc = Encoding :: for_label (label . as_ref ()) . ok_or_else (| | for_label :: Error :: Unknown { name : label . into () }) ? ; Ok (enc) }
    };
}

for_label!()
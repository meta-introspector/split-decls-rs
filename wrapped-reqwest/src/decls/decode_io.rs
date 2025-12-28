macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! decode_io {
    () => {
        deps!();
        # [allow (unused)] pub (crate) fn decode_io (e : io :: Error) -> Error { if e . get_ref () . map (| r | r . is :: < Error > ()) . unwrap_or (false) { * e . into_inner () . expect ("io::Error::get_ref was Some(_)") . downcast :: < Error > () . expect ("StdError::is() was true") } else { decode (e) } }
    };
}

decode_io!();
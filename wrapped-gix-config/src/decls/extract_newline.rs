macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! extract_newline {
    () => {
        deps!();
        pub (crate) fn extract_newline < 'a > (e : & 'a Event < '_ >) -> Option < & 'a BStr > { Some (match e { Event :: Newline (b) => { let nl = b . as_ref () ; if nl . contains (& b'\r') { "\r\n" . into () } else { "\n" . into () } } _ => return None , }) }
    };
}

extract_newline!();
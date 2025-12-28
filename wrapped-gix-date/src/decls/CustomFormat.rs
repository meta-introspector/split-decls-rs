macro_rules! CustomFormat {
    () => {
        # [doc = " A custom format for printing and parsing time."] # [derive (Clone , Copy , Debug)] pub struct CustomFormat (pub (crate) & 'static str) ;
    };
}

CustomFormat!();
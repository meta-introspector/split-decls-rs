macro_rules! FindSubstring {
    () => {
        # [doc = " Look for a substring in self"] pub trait FindSubstring < T > { # [doc = " Returns the byte position of the substring if it is found"] fn find_substring (& self , substr : T) -> Option < usize > ; }
    };
}

FindSubstring!()
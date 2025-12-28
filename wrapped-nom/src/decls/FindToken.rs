macro_rules! FindToken {
    () => {
        # [doc = " Look for a token in self"] pub trait FindToken < T > { # [doc = " Returns true if self contains the token"] fn find_token (& self , token : T) -> bool ; }
    };
}

FindToken!();
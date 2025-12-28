macro_rules! Flag {
    () => {
        # [doc = " A single flag."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum Flag { # [doc = " `i`"] CaseInsensitive , # [doc = " `m`"] MultiLine , # [doc = " `s`"] DotMatchesNewLine , # [doc = " `U`"] SwapGreed , # [doc = " `u`"] Unicode , # [doc = " `R`"] CRLF , # [doc = " `x`"] IgnoreWhitespace , }
    };
}

Flag!()
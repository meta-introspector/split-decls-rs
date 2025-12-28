macro_rules! ClassPerlKind {
    () => {
        # [doc = " The available Perl character classes."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum ClassPerlKind { # [doc = " Decimal numbers."] Digit , # [doc = " Whitespace."] Space , # [doc = " Word characters."] Word , }
    };
}

ClassPerlKind!()
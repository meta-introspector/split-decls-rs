macro_rules! CharSpecifier {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] enum CharSpecifier { SingleChar (char) , CharRange (char , char) , }
    };
}

CharSpecifier!();
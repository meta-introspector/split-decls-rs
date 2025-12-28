macro_rules! LetKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum LetKind { Regular , Super , }
    };
}

LetKind!();
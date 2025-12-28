macro_rules! Options {
    () => {
        # [doc = " Options for serializing an abstract syntax tree."] # [derive (Clone , Copy , Debug , Default , Eq , PartialEq)] pub struct Options { # [doc = " Whether invalid text fragments should be serialized, too."] pub with_junk : bool , }
    };
}

Options!()
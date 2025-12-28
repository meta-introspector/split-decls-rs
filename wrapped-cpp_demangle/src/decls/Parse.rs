macro_rules! deps {
    () => {
        Result!();
        IndexStr!();
        SubstitutionTable!();
        ParseContext!();
    };
}

macro_rules! Parse {
    () => {
        deps!();
        # [doc = " A trait for anything that can be parsed from an `IndexStr` and return a"] # [doc = " `Result` of the parsed `Self` value and the rest of the `IndexStr` input"] # [doc = " that has not been consumed in parsing the `Self` value."] # [doc = ""] # [doc = " For AST types representing productions which have `<substitution>` as a"] # [doc = " possible right hand side, do not implement this trait directly. Instead,"] # [doc = " make a newtype over `usize`, parse either the `<substitution>` back"] # [doc = " reference or \"real\" value, insert the \"real\" value into the substitution"] # [doc = " table if needed, and *always* return the newtype index into the substitution"] # [doc = " table."] # [doc (hidden)] pub trait Parse : Sized { # [doc = " Parse the `Self` value from `input` and return it, updating the"] # [doc = " substitution table as needed."] fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Self , IndexStr < 'b >) > ; }
    };
}

Parse!()
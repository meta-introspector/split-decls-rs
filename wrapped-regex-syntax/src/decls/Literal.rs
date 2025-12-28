macro_rules! deps {
    () => {
        Hir!();
    };
}

macro_rules! Literal {
    () => {
        deps!();
        # [doc = " The high-level intermediate representation of a literal."] # [doc = ""] # [doc = " A literal corresponds to `0` or more bytes that should be matched"] # [doc = " literally. The smart constructors defined on `Hir` will automatically"] # [doc = " concatenate adjacent literals into one literal, and will even automatically"] # [doc = " replace empty literals with `Hir::empty()`."] # [doc = ""] # [doc = " Note that despite a literal being represented by a sequence of bytes, its"] # [doc = " `Debug` implementation will attempt to print it as a normal string. (That"] # [doc = " is, not a sequence of decimal numbers.)"] # [derive (Clone , Eq , PartialEq)] pub struct Literal (pub Box < [u8] >) ;
    };
}

Literal!();
macro_rules! deps {
    () => {
        Hir!();
        Alternation!();
        Class!();
        Repetition!();
        Look!();
        Capture!();
        Literal!();
        Concat!();
    };
}

macro_rules! HirKind {
    () => {
        deps!();
        # [doc = " The underlying kind of an arbitrary [`Hir`] expression."] # [doc = ""] # [doc = " An `HirKind` is principally useful for doing case analysis on the type"] # [doc = " of a regular expression. If you're looking to build new `Hir` values,"] # [doc = " then you _must_ use the smart constructors defined on `Hir`, like"] # [doc = " [`Hir::repetition`], to build new `Hir` values. The API intentionally does"] # [doc = " not expose any way of building an `Hir` directly from an `HirKind`."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum HirKind { # [doc = " The empty regular expression, which matches everything, including the"] # [doc = " empty string."] Empty , # [doc = " A literal string that matches exactly these bytes."] Literal (Literal) , # [doc = " A single character class that matches any of the characters in the"] # [doc = " class. A class can either consist of Unicode scalar values as"] # [doc = " characters, or it can use bytes."] # [doc = ""] # [doc = " A class may be empty. In which case, it matches nothing."] Class (Class) , # [doc = " A look-around assertion. A look-around match always has zero length."] Look (Look) , # [doc = " A repetition operation applied to a sub-expression."] Repetition (Repetition) , # [doc = " A capturing group, which contains a sub-expression."] Capture (Capture) , # [doc = " A concatenation of expressions."] # [doc = ""] # [doc = " A concatenation matches only if each of its sub-expressions match one"] # [doc = " after the other."] # [doc = ""] # [doc = " Concatenations are guaranteed by `Hir`'s smart constructors to always"] # [doc = " have at least two sub-expressions."] Concat (Vec < Hir >) , # [doc = " An alternation of expressions."] # [doc = ""] # [doc = " An alternation matches only if at least one of its sub-expressions"] # [doc = " match. If multiple sub-expressions match, then the leftmost is"] # [doc = " preferred."] # [doc = ""] # [doc = " Alternations are guaranteed by `Hir`'s smart constructors to always"] # [doc = " have at least two sub-expressions."] Alternation (Vec < Hir >) , }
    };
}

HirKind!()
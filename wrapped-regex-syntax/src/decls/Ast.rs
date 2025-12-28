macro_rules! deps {
    () => {
        Group!();
        Concat!();
        Flags!();
        ClassUnicode!();
        Assertion!();
        Alternation!();
        ClassPerl!();
        ClassBracketed!();
        Literal!();
        Repetition!();
        SetFlags!();
        Span!();
        Dot!();
    };
}

macro_rules! Ast {
    () => {
        deps!();
        # [doc = " An abstract syntax tree for a single regular expression."] # [doc = ""] # [doc = " An `Ast`'s `fmt::Display` implementation uses constant stack space and heap"] # [doc = " space proportional to the size of the `Ast`."] # [doc = ""] # [doc = " This type defines its own destructor that uses constant stack space and"] # [doc = " heap space proportional to the size of the `Ast`."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum Ast { # [doc = " An empty regex that matches everything."] Empty (Box < Span >) , # [doc = " A set of flags, e.g., `(?is)`."] Flags (Box < SetFlags >) , # [doc = " A single character literal, which includes escape sequences."] Literal (Box < Literal >) , # [doc = " The \"any character\" class."] Dot (Box < Span >) , # [doc = " A single zero-width assertion."] Assertion (Box < Assertion >) , # [doc = " A single Unicode character class, e.g., `\\pL` or `\\p{Greek}`."] ClassUnicode (Box < ClassUnicode >) , # [doc = " A single perl character class, e.g., `\\d` or `\\W`."] ClassPerl (Box < ClassPerl >) , # [doc = " A single bracketed character class set, which may contain zero or more"] # [doc = " character ranges and/or zero or more nested classes. e.g.,"] # [doc = " `[a-zA-Z\\pL]`."] ClassBracketed (Box < ClassBracketed >) , # [doc = " A repetition operator applied to an arbitrary regular expression."] Repetition (Box < Repetition >) , # [doc = " A grouped regular expression."] Group (Box < Group >) , # [doc = " An alternation of regular expressions."] Alternation (Box < Alternation >) , # [doc = " A concatenation of regular expressions."] Concat (Box < Concat >) , }
    };
}

Ast!();
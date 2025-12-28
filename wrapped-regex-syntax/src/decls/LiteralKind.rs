macro_rules! deps {
    () => {
        SpecialLiteralKind!();
        HexLiteralKind!();
    };
}

macro_rules! LiteralKind {
    () => {
        deps!();
        # [doc = " The kind of a single literal expression."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum LiteralKind { # [doc = " The literal is written verbatim, e.g., `a` or `☃`."] Verbatim , # [doc = " The literal is written as an escape because it is otherwise a special"] # [doc = " regex meta character, e.g., `\\*` or `\\[`."] Meta , # [doc = " The literal is written as an escape despite the fact that the escape is"] # [doc = " unnecessary, e.g., `\\%` or `\\/`."] Superfluous , # [doc = " The literal is written as an octal escape, e.g., `\\141`."] Octal , # [doc = " The literal is written as a hex code with a fixed number of digits"] # [doc = " depending on the type of the escape, e.g., `\\x61` or `\\u0061` or"] # [doc = " `\\U00000061`."] HexFixed (HexLiteralKind) , # [doc = " The literal is written as a hex code with a bracketed number of"] # [doc = " digits. The only restriction is that the bracketed hex code must refer"] # [doc = " to a valid Unicode scalar value."] HexBrace (HexLiteralKind) , # [doc = " The literal is written as a specially recognized escape, e.g., `\\f`"] # [doc = " or `\\n`."] Special (SpecialLiteralKind) , }
    };
}

LiteralKind!();
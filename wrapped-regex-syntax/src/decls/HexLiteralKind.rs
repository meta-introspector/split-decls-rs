macro_rules! HexLiteralKind {
    () => {
        # [doc = " The type of a Unicode hex literal."] # [doc = ""] # [doc = " Note that all variants behave the same when used with brackets. They only"] # [doc = " differ when used without brackets in the number of hex digits that must"] # [doc = " follow."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum HexLiteralKind { # [doc = " A `\\x` prefix. When used without brackets, this form is limited to"] # [doc = " two digits."] X , # [doc = " A `\\u` prefix. When used without brackets, this form is limited to"] # [doc = " four digits."] UnicodeShort , # [doc = " A `\\U` prefix. When used without brackets, this form is limited to"] # [doc = " eight digits."] UnicodeLong , }
    };
}

HexLiteralKind!();
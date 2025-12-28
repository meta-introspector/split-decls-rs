macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! MacStmtStyle {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , Walkable)] pub enum MacStmtStyle { # [doc = " The macro statement had a trailing semicolon (e.g., `foo! { ... };`"] # [doc = " `foo!(...);`, `foo![...];`)."] Semicolon , # [doc = " The macro statement had braces (e.g., `foo! { ... }`)."] Braces , # [doc = " The macro statement had parentheses or brackets and no semicolon (e.g.,"] # [doc = " `foo!(...)`). All of these will end up being converted into macro"] # [doc = " expressions."] NoBraces , }
    };
}

MacStmtStyle!();
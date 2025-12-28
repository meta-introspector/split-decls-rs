macro_rules! Pos {
    () => {
        # [doc = " Original position of an element in source code."] # [doc = ""] # [doc = " You can serialize and deserialize it to the GraphQL `locations` format"] # [doc = " ([reference](https://spec.graphql.org/October2021/#sec-Errors))."] # [derive (PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Default , Hash , Serialize , Deserialize)] pub struct Pos { # [doc = " One-based line number."] pub line : usize , # [doc = " One-based column number."] pub column : usize , }
    };
}

Pos!();
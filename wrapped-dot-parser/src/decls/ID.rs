macro_rules! ID {
    () => {
        # [doc = " An identifier, i.e. \"ID\" in the grammar. Essentially a string."] # [doc = ""] # [doc = " Identifiers are informally presented in the specification. They can be:"] # [doc = "  - Any string of alphabetic, digit, or underscore, not begining with a digit"] # [doc = "  - A numeral"] # [doc = "  - A double-quoted string"] # [doc = "  - An HTML string"] # [doc = ""] # [doc = " This structure is simply a wrapper over &str. Depending on how the string is specified, parsing"] # [doc = " changes a bit. For instance, when a double-quoted string is used, leading and trailing quote"] # [doc = " marks are dropped."] # [doc = " HTML strings are not properly supported."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash , Clone)] pub struct ID < 'a > (& 'a str) ;
    };
}

ID!()
macro_rules! deps {
    () => {
        ClassUnicodeOpKind!();
    };
}

macro_rules! ClassUnicodeKind {
    () => {
        deps!();
        # [doc = " The available forms of Unicode character classes."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ClassUnicodeKind { # [doc = " A one letter abbreviated class, e.g., `\\pN`."] OneLetter (char) , # [doc = " A binary property, general category or script. The string may be"] # [doc = " empty."] Named (String) , # [doc = " A property name and an associated value."] NamedValue { # [doc = " The type of Unicode op used to associate `name` with `value`."] op : ClassUnicodeOpKind , # [doc = " The property name (which may be empty)."] name : String , # [doc = " The property value (which may be empty)."] value : String , } , }
    };
}

ClassUnicodeKind!();
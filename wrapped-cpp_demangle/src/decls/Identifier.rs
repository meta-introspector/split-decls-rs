macro_rules! Identifier {
    () => {
        # [doc = " The `<identifier>` pseudo-terminal."] # [doc = ""] # [doc = " ```text"] # [doc = " <identifier> ::= <unqualified source code identifier>"] # [doc = " ```"] # [doc = ""] # [doc = " > `<identifier>` is a pseudo-terminal representing the characters in the"] # [doc = " > unqualified identifier for the entity in the source code. This ABI does not"] # [doc = " > yet specify a mangling for identifiers containing characters outside of"] # [doc = " > `_A-Za-z0-9.`."] # [doc = ""] # [doc = " Mangled symbols' identifiers also have `$` characters in the wild."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Identifier { start : usize , end : usize , }
    };
}

Identifier!();
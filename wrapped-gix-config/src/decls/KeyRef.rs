macro_rules! KeyRef {
    () => {
        # [doc = " An unvalidated parse result of parsing input like `remote.origin.url` or `core.bare`."] # [derive (Debug , PartialEq , Ord , PartialOrd , Eq , Hash , Clone , Copy)] pub struct KeyRef < 'a > { # [doc = " The name of the section, like `core` in `core.bare`."] pub section_name : & 'a str , # [doc = " The name of the subsection, like `origin` in `remote.origin.url`."] pub subsection_name : Option < & 'a BStr > , # [doc = " The name of the section key, like `url` in `remote.origin.url`."] pub value_name : & 'a str , }
    };
}

KeyRef!();
macro_rules! Key {
    () => {
        # [doc = " An unvalidated parse result of a key for a section, parsing input like `remote.origin` or `core`."] # [derive (Debug , PartialEq , Ord , PartialOrd , Eq , Hash , Clone , Copy)] pub struct Key < 'a > { # [doc = " The name of the section, like `remote` in `remote.origin`."] pub section_name : & 'a str , # [doc = " The name of the sub-section, like `origin` in `remote.origin`."] pub subsection_name : Option < & 'a BStr > , }
    };
}

Key!()
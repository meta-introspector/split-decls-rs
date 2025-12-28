macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! PeImportNameType {
    () => {
        deps!();
        # [doc = " Different ways that the PE Format can decorate a symbol name."] # [doc = " From <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-name-type>"] # [derive (Copy , Clone , Debug , Encodable , Decodable , HashStable_Generic , PartialEq , Eq , PrintAttribute)] pub enum PeImportNameType { # [doc = " IMPORT_ORDINAL"] # [doc = " Uses the ordinal (i.e., a number) rather than the name."] Ordinal (u16) , # [doc = " Same as IMPORT_NAME"] # [doc = " Name is decorated with all prefixes and suffixes."] Decorated , # [doc = " Same as IMPORT_NAME_NOPREFIX"] # [doc = " Prefix (e.g., the leading `_` or `@`) is skipped, but suffix is kept."] NoPrefix , # [doc = " Same as IMPORT_NAME_UNDECORATE"] # [doc = " Prefix (e.g., the leading `_` or `@`) and suffix (the first `@` and all"] # [doc = " trailing characters) are skipped."] Undecorated , }
    };
}

PeImportNameType!();
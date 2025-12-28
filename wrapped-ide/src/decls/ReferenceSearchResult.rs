macro_rules! deps {
    () => {
        Declaration!();
    };
}

macro_rules! ReferenceSearchResult {
    () => {
        deps!();
        # [doc = " Result of a reference search operation."] # [derive (Debug , Clone , UpmapFromRaFixture)] pub struct ReferenceSearchResult { # [doc = " Information about the declaration site of the searched item."] # [doc = " For ADTs (structs/enums), this points to the type definition."] # [doc = " May be None for primitives or items without clear declaration sites."] pub declaration : Option < Declaration > , # [doc = " All references found, grouped by file."] # [doc = " For ADTs when searching from a constructor position (e.g. on '{', '(', ';'),"] # [doc = " this only includes constructor/initialization usages."] # [doc = " The map key is the file ID, and the value is a vector of (range, category) pairs."] # [doc = " - range: The text range of the reference in the file"] # [doc = " - category: Metadata about how the reference is used (read/write/etc)"] pub references : IntMap < FileId , Vec < (TextRange , ReferenceCategory) > > , }
    };
}

ReferenceSearchResult!();
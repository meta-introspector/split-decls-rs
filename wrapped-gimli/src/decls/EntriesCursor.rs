macro_rules! deps {
    () => {
        Reader!();
        DebuggingInformationEntry!();
        Abbreviations!();
        UnitHeader!();
    };
}

macro_rules! EntriesCursor {
    () => {
        deps!();
        # [doc = " A cursor into the Debugging Information Entries tree for a compilation unit."] # [doc = ""] # [doc = " The `EntriesCursor` can traverse the DIE tree in DFS order using `next_dfs()`,"] # [doc = " or skip to the next sibling of the entry the cursor is currently pointing to"] # [doc = " using `next_sibling()`."] # [doc = ""] # [doc = " It is also possible to traverse the DIE tree at a lower abstraction level"] # [doc = " using `next_entry()`. This method does not skip over null entries, or provide"] # [doc = " any indication of the current tree depth. In this case, you must use `current()`"] # [doc = " to obtain the current entry, and `current().has_children()` to determine if"] # [doc = " the entry following the current entry will be a sibling or child. `current()`"] # [doc = " will return `None` if the current entry is a null entry, which signifies the"] # [doc = " end of the current tree depth."] # [derive (Clone , Debug)] pub struct EntriesCursor < 'abbrev , 'unit , R > where R : Reader , { input : R , unit : & 'unit UnitHeader < R > , abbreviations : & 'abbrev Abbreviations , cached_current : Option < DebuggingInformationEntry < 'abbrev , 'unit , R > > , delta_depth : isize , }
    };
}

EntriesCursor!();
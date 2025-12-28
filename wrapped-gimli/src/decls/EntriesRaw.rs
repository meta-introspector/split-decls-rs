macro_rules! deps {
    () => {
        EntriesCursor!();
        Reader!();
        UnitHeader!();
        Abbreviations!();
        Result!();
    };
}

macro_rules! EntriesRaw {
    () => {
        deps!();
        # [doc = " A raw reader of the data that defines the Debugging Information Entries."] # [doc = ""] # [doc = " `EntriesRaw` provides primitives to read the components of Debugging Information"] # [doc = " Entries (DIEs). A DIE consists of an abbreviation code (read with `read_abbreviation`)"] # [doc = " followed by a number of attributes (read with `read_attribute`)."] # [doc = " The user must provide the control flow to read these correctly."] # [doc = " In particular, all attributes must always be read before reading another"] # [doc = " abbreviation code."] # [doc = ""] # [doc = " `EntriesRaw` lacks some features of `EntriesCursor`, such as the ability to skip"] # [doc = " to the next sibling DIE. However, this also allows it to optimize better, since it"] # [doc = " does not need to perform the extra bookkeeping required to support these features,"] # [doc = " and thus it is suitable for cases where performance is important."] # [doc = ""] # [doc = " ## Example Usage"] # [doc = " ```rust,no_run"] # [doc = " # fn example() -> Result<(), gimli::Error> {"] # [doc = " # let debug_info = gimli::DebugInfo::new(&[], gimli::LittleEndian);"] # [doc = " # let get_some_unit = || debug_info.units().next().unwrap().unwrap();"] # [doc = " let unit = get_some_unit();"] # [doc = " # let debug_abbrev = gimli::DebugAbbrev::new(&[], gimli::LittleEndian);"] # [doc = " # let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();"] # [doc = " let abbrevs = get_abbrevs_for_unit(&unit);"] # [doc = ""] # [doc = " let mut entries = unit.entries_raw(&abbrevs, None)?;"] # [doc = " while !entries.is_empty() {"] # [doc = "     let abbrev = if let Some(abbrev) = entries.read_abbreviation()? {"] # [doc = "         abbrev"] # [doc = "     } else {"] # [doc = "         // Null entry with no attributes."] # [doc = "         continue"] # [doc = "     };"] # [doc = "     match abbrev.tag() {"] # [doc = "         gimli::DW_TAG_subprogram => {"] # [doc = "             // Loop over attributes for DIEs we care about."] # [doc = "             for spec in abbrev.attributes() {"] # [doc = "                 let attr = entries.read_attribute(*spec)?;"] # [doc = "                 match attr.name() {"] # [doc = "                     // Handle attributes."] # [doc = "                     _ => {}"] # [doc = "                 }"] # [doc = "             }"] # [doc = "         }"] # [doc = "         _ => {"] # [doc = "             // Skip attributes for DIEs we don't care about."] # [doc = "             entries.skip_attributes(abbrev.attributes());"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " # unreachable!()"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct EntriesRaw < 'abbrev , 'unit , R > where R : Reader , { input : R , unit : & 'unit UnitHeader < R > , abbreviations : & 'abbrev Abbreviations , depth : isize , }
    };
}

EntriesRaw!();
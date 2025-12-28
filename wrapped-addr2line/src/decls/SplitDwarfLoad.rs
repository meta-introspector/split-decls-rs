macro_rules! SplitDwarfLoad {
    () => {
        # [doc = " This struct contains the information needed to find split DWARF data"] # [doc = " and to produce a `gimli::Dwarf<R>` for it."] pub struct SplitDwarfLoad < R > { # [doc = " The dwo id, for looking up in a DWARF package, or for"] # [doc = " verifying an unpacked dwo found on the file system"] pub dwo_id : gimli :: DwoId , # [doc = " The compilation directory `path` is relative to."] pub comp_dir : Option < R > , # [doc = " A path on the filesystem, relative to `comp_dir` to find this dwo."] pub path : Option < R > , # [doc = " Once the split DWARF data is loaded, the loader is expected"] # [doc = " to call [make_dwo(parent)](gimli::read::Dwarf::make_dwo) before"] # [doc = " returning the data."] pub parent : Arc < gimli :: Dwarf < R > > , }
    };
}

SplitDwarfLoad!()
macro_rules! deps {
    () => {
        SourceRoot!();
        CrateGraphBuilder!();
    };
}

macro_rules! FileChange {
    () => {
        deps!();
        # [doc = " Encapsulate a bunch of raw `.set` calls on the database."] # [derive (Default)] pub struct FileChange { pub roots : Option < Vec < SourceRoot > > , pub files_changed : Vec < (FileId , Option < String >) > , pub crate_graph : Option < CrateGraphBuilder > , }
    };
}

FileChange!()
macro_rules! NonExhaustiveLet {
    () => {
        # [derive (Debug)] pub struct NonExhaustiveLet { pub pat : InFile < AstPtr < ast :: Pat > > , pub uncovered_patterns : String , }
    };
}

NonExhaustiveLet!();
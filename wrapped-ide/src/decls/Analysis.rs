macro_rules! Analysis {
    () => {
        # [doc = " Analysis is a snapshot of a world state at a moment in time. It is the main"] # [doc = " entry point for asking semantic information about the world. When the world"] # [doc = " state is advanced using `AnalysisHost::apply_change` method, all existing"] # [doc = " `Analysis` are canceled (most method return `Err(Canceled)`)."] # [derive (Debug)] pub struct Analysis { db : RootDatabase , }
    };
}

Analysis!();
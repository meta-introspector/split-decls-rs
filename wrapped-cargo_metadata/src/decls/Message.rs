macro_rules! deps {
    () => {
        CompilerMessage!();
        BuildFinished!();
        Artifact!();
        BuildScript!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [doc = " A cargo message"] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [non_exhaustive] # [serde (tag = "reason" , rename_all = "kebab-case")] pub enum Message { # [doc = " The compiler generated an artifact"] CompilerArtifact (Artifact) , # [doc = " The compiler wants to display a message"] CompilerMessage (CompilerMessage) , # [doc = " A build script successfully executed."] BuildScriptExecuted (BuildScript) , # [doc = " The build has finished."] # [doc = ""] # [doc = " This is emitted at the end of the build as the last message."] # [doc = " Added in Rust 1.44."] BuildFinished (BuildFinished) , # [doc = " A line of text which isn't a cargo or compiler message."] # [doc = " Line separator is not included"] # [serde (skip)] TextLine (String) , }
    };
}

Message!()
macro_rules! BreakableKind {
    () => {
        # [derive (Clone , Debug)] enum BreakableKind { Block , Loop , # [doc = " A border is something like an async block, closure etc. Anything that prevents"] # [doc = " breaking/continuing through"] Border , }
    };
}

BreakableKind!()
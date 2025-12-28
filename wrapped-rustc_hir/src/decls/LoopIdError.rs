macro_rules! LoopIdError {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , HashStable_Generic)] pub enum LoopIdError { OutsideLoopScope , UnlabeledCfInWhileCondition , UnresolvedLabel , }
    };
}

LoopIdError!()
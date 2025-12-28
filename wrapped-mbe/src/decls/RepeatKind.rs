macro_rules! RepeatKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum RepeatKind { ZeroOrMore , OneOrMore , ZeroOrOne , }
    };
}

RepeatKind!();
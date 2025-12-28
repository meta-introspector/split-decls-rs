macro_rules! VariableKind {
    () => {
        # [derive (Copy , Clone)] pub enum VariableKind { ArgumentVariable (usize) , LocalVariable , }
    };
}

VariableKind!()
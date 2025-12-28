macro_rules! deps {
    () => {
        QualifiedBuiltin!();
        BuiltinType!();
        Type!();
    };
}

macro_rules! macro_156 {
    () => {
        deps!();
        define_handle ! { # [doc = " A reference to a parsed `Type` production."] pub enum TypeHandle { # [doc = " A builtin type. These don't end up in the substitutions table."] extra Builtin (BuiltinType) , # [doc = " A CV-qualified builtin type. These don't end up in the table either."] extra QualifiedBuiltin (QualifiedBuiltin) , } }
    };
}

macro_156!()
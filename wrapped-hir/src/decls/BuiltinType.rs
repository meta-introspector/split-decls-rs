macro_rules! BuiltinType {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct BuiltinType { pub (crate) inner : hir_def :: builtin_type :: BuiltinType , }
    };
}

BuiltinType!()
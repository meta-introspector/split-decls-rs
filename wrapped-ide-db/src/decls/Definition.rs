macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! Definition {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Copy , Clone , Hash)] pub enum Definition { Macro (Macro) , Field (Field) , TupleField (TupleField) , Module (Module) , Crate (Crate) , Function (Function) , Adt (Adt) , Variant (Variant) , Const (Const) , Static (Static) , Trait (Trait) , TypeAlias (TypeAlias) , SelfType (Impl) , GenericParam (GenericParam) , Local (Local) , Label (Label) , DeriveHelper (DeriveHelper) , BuiltinType (BuiltinType) , BuiltinLifetime (StaticLifetime) , BuiltinAttr (BuiltinAttr) , ToolModule (ToolModule) , ExternCrateDecl (ExternCrateDecl) , InlineAsmRegOrRegClass (()) , InlineAsmOperand (InlineAsmOperand) , }
    };
}

Definition!();
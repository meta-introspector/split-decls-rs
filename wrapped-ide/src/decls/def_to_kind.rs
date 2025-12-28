macro_rules! deps {
    () => {
        SymbolInformationKind!();
    };
}

macro_rules! def_to_kind {
    () => {
        deps!();
        pub (crate) fn def_to_kind (db : & RootDatabase , def : Definition) -> SymbolInformationKind { use SymbolInformationKind :: * ; match def { Definition :: Macro (it) => match it . kind (db) { MacroKind :: Derive | MacroKind :: DeriveBuiltIn | MacroKind :: AttrBuiltIn | MacroKind :: Attr => Attribute , MacroKind :: Declarative | MacroKind :: DeclarativeBuiltIn | MacroKind :: ProcMacro => Macro , } , Definition :: Field (..) | Definition :: TupleField (..) => Field , Definition :: Module (..) | Definition :: Crate (..) => Module , Definition :: Function (it) => { if it . as_assoc_item (db) . is_some () { if it . has_self_param (db) { if it . has_body (db) { Method } else { TraitMethod } } else { StaticMethod } } else { Function } } Definition :: Adt (Adt :: Struct (..)) => Struct , Definition :: Adt (Adt :: Union (..)) => Union , Definition :: Adt (Adt :: Enum (..)) => Enum , Definition :: Variant (..) => EnumMember , Definition :: Const (..) => Constant , Definition :: Static (..) => StaticVariable , Definition :: Trait (..) => Trait , Definition :: TypeAlias (it) => { if it . as_assoc_item (db) . is_some () { AssociatedType } else { TypeAlias } } Definition :: BuiltinType (..) => Type , Definition :: BuiltinLifetime (_) => TypeParameter , Definition :: SelfType (..) => TypeAlias , Definition :: GenericParam (..) => TypeParameter , Definition :: Local (it) => { if it . is_self (db) { SelfParameter } else if it . is_param (db) { Parameter } else { Variable } } Definition :: Label (..) | Definition :: InlineAsmOperand (_) => Variable , Definition :: DeriveHelper (..) => Attribute , Definition :: BuiltinAttr (..) => Attribute , Definition :: ToolModule (..) => Module , Definition :: ExternCrateDecl (..) => Module , Definition :: InlineAsmRegOrRegClass (..) => Module , } }
    };
}

def_to_kind!()
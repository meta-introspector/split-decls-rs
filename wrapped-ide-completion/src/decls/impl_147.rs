macro_rules! deps {
    () => {
        CompletionItemKind!();
        Snippet!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl CompletionItemKind { pub fn tag (self) -> & 'static str { match self { CompletionItemKind :: SymbolKind (kind) => match kind { SymbolKind :: Attribute => "at" , SymbolKind :: BuiltinAttr => "ba" , SymbolKind :: Const => "ct" , SymbolKind :: ConstParam => "cp" , SymbolKind :: Derive => "de" , SymbolKind :: DeriveHelper => "dh" , SymbolKind :: Enum => "en" , SymbolKind :: Field => "fd" , SymbolKind :: Function => "fn" , SymbolKind :: Impl => "im" , SymbolKind :: InlineAsmRegOrRegClass => "ar" , SymbolKind :: Label => "lb" , SymbolKind :: LifetimeParam => "lt" , SymbolKind :: Local => "lc" , SymbolKind :: Macro => "ma" , SymbolKind :: Method => "me" , SymbolKind :: ProcMacro => "pm" , SymbolKind :: Module => "md" , SymbolKind :: SelfParam => "sp" , SymbolKind :: SelfType => "sy" , SymbolKind :: Static => "sc" , SymbolKind :: Struct => "st" , SymbolKind :: ToolModule => "tm" , SymbolKind :: Trait => "tt" , SymbolKind :: TypeAlias => "ta" , SymbolKind :: TypeParam => "tp" , SymbolKind :: Union => "un" , SymbolKind :: ValueParam => "vp" , SymbolKind :: Variant => "ev" , } , CompletionItemKind :: Binding => "bn" , CompletionItemKind :: BuiltinType => "bt" , CompletionItemKind :: InferredType => "it" , CompletionItemKind :: Keyword => "kw" , CompletionItemKind :: Snippet => "sn" , CompletionItemKind :: UnresolvedReference => "??" , CompletionItemKind :: Expression => "ex" , } } }
    };
}

impl_147!()
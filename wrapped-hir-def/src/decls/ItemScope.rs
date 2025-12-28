macro_rules! deps {
    () => {
        ImportOrExternCrate!();
        TypesItem!();
        ImportOrDef!();
        ModuleDefId!();
        FxIndexMap!();
        Item!();
        ImportOrGlob!();
        MacroCall!();
        ValuesItem!();
        Trait!();
        MacroId!();
        MacrosItem!();
        DeriveMacroInvocation!();
    };
}

macro_rules! ItemScope {
    () => {
        deps!();
        # [derive (Debug , Default , PartialEq , Eq)] pub struct ItemScope { # [doc = " Defs visible in this scope. This includes `declarations`, but also"] # [doc = " imports. The imports belong to this module and can be resolved by using them on"] # [doc = " the `use_imports_*` fields."] types : FxIndexMap < Name , TypesItem > , values : FxIndexMap < Name , ValuesItem > , macros : FxIndexMap < Name , MacrosItem > , unresolved : FxHashSet < Name > , # [doc = " The defs declared in this scope. Each def has a single scope where it is"] # [doc = " declared."] declarations : ThinVec < ModuleDefId > , impls : ThinVec < ImplId > , extern_blocks : ThinVec < ExternBlockId > , unnamed_consts : ThinVec < ConstId > , # [doc = " Traits imported via `use Trait as _;`."] unnamed_trait_imports : ThinVec < (TraitId , Item < () >) > , use_imports_types : FxHashMap < ImportOrExternCrate , ImportOrDef > , use_imports_values : FxHashMap < ImportOrGlob , ImportOrDef > , use_imports_macros : FxHashMap < ImportOrExternCrate , ImportOrDef > , use_decls : ThinVec < UseId > , extern_crate_decls : ThinVec < ExternCrateId > , # [doc = " Macros visible in current module in legacy textual scope"] # [doc = ""] # [doc = " For macros invoked by an unqualified identifier like `bar!()`, `legacy_macros` will be searched in first."] # [doc = " If it yields no result, then it turns to module scoped `macros`."] # [doc = " It macros with name qualified with a path like `crate::foo::bar!()`, `legacy_macros` will be skipped,"] # [doc = " and only normal scoped `macros` will be searched in."] # [doc = ""] # [doc = " Note that this automatically inherit macros defined textually before the definition of module itself."] # [doc = ""] # [doc = " Module scoped macros will be inserted into `items` instead of here."] legacy_macros : FxHashMap < Name , SmallVec < MacroId , 2 > > , # [doc = " The attribute macro invocations in this scope."] attr_macros : FxHashMap < AstId < ast :: Item > , MacroCallId > , # [doc = " The macro invocations in this scope."] macro_invocations : FxHashMap < AstId < ast :: MacroCall > , MacroCallId > , # [doc = " The derive macro invocations in this scope, keyed by the owner item over the actual derive attributes"] # [doc = " paired with the derive macro invocations for the specific attribute."] derive_macros : FxHashMap < AstId < ast :: Adt > , SmallVec < DeriveMacroInvocation , 1 > > , }
    };
}

ItemScope!()
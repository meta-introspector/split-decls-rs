macro_rules! deps {
    () => {
        ProcMacroLoc!();
        StaticLoc!();
        BlockLoc!();
        ExternBlockLoc!();
        UnionLoc!();
        FunctionLoc!();
        StructLoc!();
        UseLoc!();
        ExternCrateLoc!();
        ImplLoc!();
        EnumLoc!();
        TraitLoc!();
        Macro2Loc!();
        EnumVariantLoc!();
        MacroRulesLoc!();
        TypeAliasLoc!();
        ConstLoc!();
    };
}

macro_rules! InternDatabase {
    () => {
        deps!();
        # [query_group :: query_group (InternDatabaseStorage)] pub trait InternDatabase : RootQueryDb { # [salsa :: interned] fn intern_use (& self , loc : UseLoc) -> UseId ; # [salsa :: interned] fn intern_extern_crate (& self , loc : ExternCrateLoc) -> ExternCrateId ; # [salsa :: interned] fn intern_function (& self , loc : FunctionLoc) -> FunctionId ; # [salsa :: interned] fn intern_struct (& self , loc : StructLoc) -> StructId ; # [salsa :: interned] fn intern_union (& self , loc : UnionLoc) -> UnionId ; # [salsa :: interned] fn intern_enum (& self , loc : EnumLoc) -> EnumId ; # [salsa :: interned] fn intern_enum_variant (& self , loc : EnumVariantLoc) -> EnumVariantId ; # [salsa :: interned] fn intern_const (& self , loc : ConstLoc) -> ConstId ; # [salsa :: interned] fn intern_static (& self , loc : StaticLoc) -> StaticId ; # [salsa :: interned] fn intern_trait (& self , loc : TraitLoc) -> TraitId ; # [salsa :: interned] fn intern_type_alias (& self , loc : TypeAliasLoc) -> TypeAliasId ; # [salsa :: interned] fn intern_impl (& self , loc : ImplLoc) -> ImplId ; # [salsa :: interned] fn intern_extern_block (& self , loc : ExternBlockLoc) -> ExternBlockId ; # [salsa :: interned] fn intern_macro2 (& self , loc : Macro2Loc) -> Macro2Id ; # [salsa :: interned] fn intern_proc_macro (& self , loc : ProcMacroLoc) -> ProcMacroId ; # [salsa :: interned] fn intern_macro_rules (& self , loc : MacroRulesLoc) -> MacroRulesId ; # [salsa :: interned] fn intern_block (& self , loc : BlockLoc) -> BlockId ; }
    };
}

InternDatabase!()
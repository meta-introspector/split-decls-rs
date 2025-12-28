macro_rules! deps {
    () => {
        Static!();
        Local!();
        Trait!();
        Function!();
        ExternBlock!();
        Adt!();
        TupleField!();
        Union!();
        Const!();
        Struct!();
        Module!();
        TypeAlias!();
        Label!();
        ExternCrateDecl!();
        ConstParam!();
        TypeParam!();
        SelfParam!();
        InlineAsmOperand!();
        Variant!();
        Enum!();
        LifetimeParam!();
        GenericParam!();
        Macro!();
        Field!();
        Impl!();
    };
}

macro_rules! macro_91 {
    () => {
        deps!();
        to_def_impls ! [(crate :: Module , ast :: Module , module_to_def) , (crate :: Module , ast :: SourceFile , source_file_to_def) , (crate :: Struct , ast :: Struct , struct_to_def) , (crate :: Enum , ast :: Enum , enum_to_def) , (crate :: Union , ast :: Union , union_to_def) , (crate :: Trait , ast :: Trait , trait_to_def) , (crate :: Impl , ast :: Impl , impl_to_def) , (crate :: TypeAlias , ast :: TypeAlias , type_alias_to_def) , (crate :: Const , ast :: Const , const_to_def) , (crate :: Static , ast :: Static , static_to_def) , (crate :: Function , ast :: Fn , fn_to_def) , (crate :: Field , ast :: RecordField , record_field_to_def) , (crate :: Field , ast :: TupleField , tuple_field_to_def) , (crate :: Variant , ast :: Variant , enum_variant_to_def) , (crate :: TypeParam , ast :: TypeParam , type_param_to_def) , (crate :: LifetimeParam , ast :: LifetimeParam , lifetime_param_to_def) , (crate :: ConstParam , ast :: ConstParam , const_param_to_def) , (crate :: GenericParam , ast :: GenericParam , generic_param_to_def) , (crate :: Macro , ast :: Macro , macro_to_def) , (crate :: Local , ast :: IdentPat , bind_pat_to_def) , (crate :: Local , ast :: SelfParam , self_param_to_def) , (crate :: Label , ast :: Label , label_to_def) , (crate :: Adt , ast :: Adt , adt_to_def) , (crate :: ExternCrateDecl , ast :: ExternCrate , extern_crate_to_def) , (crate :: InlineAsmOperand , ast :: AsmOperandNamed , asm_operand_to_def) , (crate :: ExternBlock , ast :: ExternBlock , extern_block_to_def) , (MacroCallId , ast :: MacroCall , macro_call_to_macro_call) ,] ;
    };
}

macro_91!()
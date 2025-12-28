macro_rules! deps {
    () => {
        InferenceTyDiagnosticSource!();
    };
}

macro_rules! InferenceDiagnostic {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone)] pub enum InferenceDiagnostic < 'db > { NoSuchField { field : ExprOrPatId , private : Option < LocalFieldId > , variant : VariantId , } , PrivateField { expr : ExprId , field : FieldId , } , PrivateAssocItem { id : ExprOrPatId , item : AssocItemId , } , UnresolvedField { expr : ExprId , receiver : Ty < 'db > , name : Name , method_with_same_name_exists : bool , } , UnresolvedMethodCall { expr : ExprId , receiver : Ty < 'db > , name : Name , # [doc = " Contains the type the field resolves to"] field_with_same_name : Option < Ty < 'db > > , assoc_func_with_same_name : Option < FunctionId > , } , UnresolvedAssocItem { id : ExprOrPatId , } , UnresolvedIdent { id : ExprOrPatId , } , BreakOutsideOfLoop { expr : ExprId , is_break : bool , bad_value_break : bool , } , MismatchedArgCount { call_expr : ExprId , expected : usize , found : usize , } , MismatchedTupleStructPatArgCount { pat : ExprOrPatId , expected : usize , found : usize , } , ExpectedFunction { call_expr : ExprId , found : Ty < 'db > , } , TypedHole { expr : ExprId , expected : Ty < 'db > , } , CastToUnsized { expr : ExprId , cast_ty : Ty < 'db > , } , InvalidCast { expr : ExprId , error : CastError , expr_ty : Ty < 'db > , cast_ty : Ty < 'db > , } , TyDiagnostic { source : InferenceTyDiagnosticSource , diag : TyLoweringDiagnostic , } , PathDiagnostic { node : ExprOrPatId , diag : PathLoweringDiagnostic , } , MethodCallIncorrectGenericsLen { expr : ExprId , provided_count : u32 , expected_count : u32 , kind : IncorrectGenericsLenKind , def : GenericDefId , } , MethodCallIncorrectGenericsOrder { expr : ExprId , param_id : GenericParamId , arg_idx : u32 , # [doc = " Whether the `GenericArgs` contains a `Self` arg."] has_self_arg : bool , } , }
    };
}

InferenceDiagnostic!();
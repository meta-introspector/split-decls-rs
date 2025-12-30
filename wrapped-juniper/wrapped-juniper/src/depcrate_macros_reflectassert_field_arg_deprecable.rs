// Generated macro for assert_field_arg_deprecable (macro)
macro_rules! Depcrate_macros_reflectassert_field_arg_deprecable {
() => {
// Module: crate::macros::reflect
// Provides: {"assert_field_arg_deprecable"}
// Dependencies: {}
# [doc = " Statically asserts whether a [`Field`]'s [`Argument`] represents a `Null`able type, so can be"] # [doc = " deprecated."] # [doc = ""] # [doc = " Must be used in conjunction with the check whether the [`Argument`] has its default value set."] # [doc (hidden)] # [macro_export] macro_rules ! assert_field_arg_deprecable { ($ ty : ty , $ scalar : ty , $ field_name : expr , $ arg_name : expr $ (,) ?) => { const _ : () = { const TY_NAME : &:: core :: primitive :: str = <$ ty as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME ; const FIELD_NAME : &:: core :: primitive :: str = $ field_name ; const ARGS : $ crate :: macros :: reflect :: Arguments = <$ ty as $ crate :: macros :: reflect :: FieldMeta < $ scalar , { $ crate :: checked_hash ! (FIELD_NAME , $ ty , $ scalar) } , >>:: ARGUMENTS ; const ARG_NAME : &:: core :: primitive :: str = $ arg_name ; const ARG : $ crate :: macros :: reflect :: Argument = $ crate :: macros :: reflect :: get_arg_by_name (ARGS , ARG_NAME) . expect ($ crate :: const_concat ! ("field `" , FIELD_NAME , "` has no argument `" , ARG_NAME , "`" ,) ,) ; if ARG . 2 % 10 != 2 { const ARG_TY : &:: core :: primitive :: str = $ crate :: format_type ! (ARG . 1 , ARG . 2) ; const ERROR_MSG : &:: core :: primitive :: str = $ crate :: const_concat ! ("argument `" , ARG_NAME , "` of `" , TY_NAME , "." , FIELD_NAME , "` field cannot be deprecated, because its type `" , ARG_TY , "` is neither `Null`able nor the default argument value is specified" ,) ; :: core :: panic ! ("{}" , ERROR_MSG) ; } } ; } ; }
};
}

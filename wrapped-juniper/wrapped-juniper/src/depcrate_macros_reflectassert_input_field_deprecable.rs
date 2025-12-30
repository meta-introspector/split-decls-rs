// Generated macro for assert_input_field_deprecable (macro)
macro_rules! Depcrate_macros_reflectassert_input_field_deprecable {
() => {
// Module: crate::macros::reflect
// Provides: {"assert_input_field_deprecable"}
// Dependencies: {}
# [doc = " Statically asserts whether an input object [`Field`] represents a `Null`able type, so can be"] # [doc = " deprecated."] # [doc = ""] # [doc = " Must be used in conjunction with the check whether the [`Field`] has its default value set."] # [doc (hidden)] # [macro_export] macro_rules ! assert_input_field_deprecable { ($ ty : ty , $ scalar : ty , $ field_name : expr $ (,) ?) => { const _ : () = { const TY_NAME : &:: core :: primitive :: str = <$ ty as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME ; const FIELD_NAME : &:: core :: primitive :: str = $ field_name ; const FIELD_TY_NAME : &:: core :: primitive :: str = <$ ty as $ crate :: macros :: reflect :: FieldMeta < $ scalar , { $ crate :: checked_hash ! (FIELD_NAME , $ ty , $ scalar) } , >>:: TYPE ; const FIELD_WRAPPED_VAL : $ crate :: macros :: reflect :: WrappedValue = <$ ty as $ crate :: macros :: reflect :: FieldMeta < $ scalar , { $ crate :: checked_hash ! (FIELD_NAME , $ ty , $ scalar) } , >>:: WRAPPED_VALUE ; if FIELD_WRAPPED_VAL % 10 != 2 { const FIELD_TY : &:: core :: primitive :: str = $ crate :: format_type ! (FIELD_TY_NAME , FIELD_WRAPPED_VAL) ; const ERROR_MSG : &:: core :: primitive :: str = $ crate :: const_concat ! ("field `" , FIELD_NAME , "` of `" , TY_NAME , "` input object cannot be deprecated, because its type `" , FIELD_TY , "` is neither `Null`able nor the default field value is specified" ,) ; :: core :: panic ! ("{}" , ERROR_MSG) ; } } ; } ; }
};
}

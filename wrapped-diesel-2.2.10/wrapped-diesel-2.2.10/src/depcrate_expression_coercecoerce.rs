// Generated macro for Coerce (struct)
macro_rules! Depcrate_expression_coerceCoerce {
() => {
// Module: crate::expression::coerce
// Provides: {"Coerce"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , QueryId , DieselNumericOps)] # [doc (hidden)] # [doc = " Coerces an expression to be another type. No checks are performed to ensure"] # [doc = " that the new type is valid in all positions that the previous type was."] # [doc = " This does not perform an actual cast, it just lies to our type system."] # [doc = ""] # [doc = " This is used for a few expressions where we know that the types are actually"] # [doc = " always interchangeable. (Examples of this include `Timestamp` vs"] # [doc = " `Timestamptz`, `VarChar` vs `Text`, and `Json` vs `Jsonb`)."] # [doc = ""] # [doc = " This struct should not be considered a general solution to equivalent types."] # [doc = " It is a short term workaround for expressions which are known to be commonly"] # [doc = " used."] pub struct Coerce < T , ST > { expr : T , _marker : PhantomData < ST > , }
};
}

// Generated macro for float_const_impl (macro)
macro_rules! Depcrate_floatfloat_const_impl {
() => {
// Module: crate::float
// Provides: {"float_const_impl"}
// Dependencies: {}
macro_rules ! float_const_impl { ($ (# [$ doc : meta] $ constant : ident ,) +) => (# [allow (non_snake_case)] pub trait FloatConst { $ (# [$ doc] fn $ constant () -> Self ;) + # [doc = "Return the full circle constant `τ`."] # [inline] fn TAU () -> Self where Self : Sized + Add < Self , Output = Self > { Self :: PI () + Self :: PI () } # [doc = "Return `log10(2.0)`."] # [inline] fn LOG10_2 () -> Self where Self : Sized + Div < Self , Output = Self > { Self :: LN_2 () / Self :: LN_10 () } # [doc = "Return `log2(10.0)`."] # [inline] fn LOG2_10 () -> Self where Self : Sized + Div < Self , Output = Self > { Self :: LN_10 () / Self :: LN_2 () } } float_const_impl ! { @ float f32 , $ ($ constant ,) + } float_const_impl ! { @ float f64 , $ ($ constant ,) + }) ; (@ float $ T : ident , $ ($ constant : ident ,) +) => (impl FloatConst for $ T { constant ! { $ ($ constant () -> $ T :: consts ::$ constant ;) + TAU () -> 6.28318530717958647692528676655900577 ; LOG10_2 () -> 0.301029995663981195213738894724493027 ; LOG2_10 () -> 3.32192809488736234787031942948939018 ; } }) ; }
};
}

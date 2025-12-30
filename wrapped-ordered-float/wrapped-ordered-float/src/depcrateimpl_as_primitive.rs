// Generated macro for impl_as_primitive (macro)
macro_rules! Depcrateimpl_as_primitive {
() => {
// Module: crate
// Provides: {"impl_as_primitive"}
// Dependencies: {}
macro_rules ! impl_as_primitive { (@ (NotNan <$ T : ty >) => $ (# [$ cfg : meta]) * impl (NotNan <$ U : ty >)) => { $ (# [$ cfg]) * impl AsPrimitive < NotNan <$ U >> for NotNan <$ T > { # [inline] fn as_ (self) -> NotNan <$ U > { unsafe { NotNan :: new_unchecked (self . 0 as $ U) } } } } ; (@ ($ T : ty) => $ (# [$ cfg : meta]) * impl (NotNan <$ U : ty >)) => { $ (# [$ cfg]) * impl AsPrimitive < NotNan <$ U >> for $ T { # [inline] fn as_ (self) -> NotNan <$ U > { NotNan (self as $ U) } } } ; (@ (NotNan <$ T : ty >) => $ (# [$ cfg : meta]) * impl ($ U : ty)) => { $ (# [$ cfg]) * impl AsPrimitive <$ U > for NotNan <$ T > { # [inline] fn as_ (self) -> $ U { self . 0 as $ U } } } ; (@ (OrderedFloat <$ T : ty >) => $ (# [$ cfg : meta]) * impl (OrderedFloat <$ U : ty >)) => { $ (# [$ cfg]) * impl AsPrimitive < OrderedFloat <$ U >> for OrderedFloat <$ T > { # [inline] fn as_ (self) -> OrderedFloat <$ U > { OrderedFloat (self . 0 as $ U) } } } ; (@ ($ T : ty) => $ (# [$ cfg : meta]) * impl (OrderedFloat <$ U : ty >)) => { $ (# [$ cfg]) * impl AsPrimitive < OrderedFloat <$ U >> for $ T { # [inline] fn as_ (self) -> OrderedFloat <$ U > { OrderedFloat (self as $ U) } } } ; (@ (OrderedFloat <$ T : ty >) => $ (# [$ cfg : meta]) * impl ($ U : ty)) => { $ (# [$ cfg]) * impl AsPrimitive <$ U > for OrderedFloat <$ T > { # [inline] fn as_ (self) -> $ U { self . 0 as $ U } } } ; ($ T : tt => { $ ($ U : tt) ,* }) => { $ (impl_as_primitive ! (@ $ T => impl $ U) ;) * } ; }
};
}

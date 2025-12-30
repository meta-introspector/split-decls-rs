// Generated macro for impl_122 (impl)
macro_rules! Depcrateimpl_122 {
() => {
// Module: crate
// Provides: {"impl_122"}
// Dependencies: {}
impl < T : PartialEq > PluralElements < T > { # [doc = " Sets the value for [`PluralCategory::Zero`]."] pub fn with_zero_value (self , zero : Option < T >) -> Self { Self (PluralElementsInner { zero : zero . filter (| t | * t != self . 0 . other) , .. self . 0 }) } # [doc = " Sets the value for [`PluralCategory::One`]."] pub fn with_one_value (self , one : Option < T >) -> Self { Self (PluralElementsInner { one : one . filter (| t | * t != self . 0 . other) , .. self . 0 }) } # [doc = " Sets the value for [`PluralCategory::Two`]."] pub fn with_two_value (self , two : Option < T >) -> Self { Self (PluralElementsInner { two : two . filter (| t | * t != self . 0 . other) , .. self . 0 }) } # [doc = " Sets the value for [`PluralCategory::Few`]."] pub fn with_few_value (self , few : Option < T >) -> Self { Self (PluralElementsInner { few : few . filter (| t | * t != self . 0 . other) , .. self . 0 }) } # [doc = " Sets the value for [`PluralCategory::Many`]."] pub fn with_many_value (self , many : Option < T >) -> Self { Self (PluralElementsInner { many : many . filter (| t | * t != self . 0 . other) , .. self . 0 }) } # [doc = " Sets the value for explicit 0."] pub fn with_explicit_zero_value (self , explicit_zero : Option < T >) -> Self { Self (PluralElementsInner { explicit_zero , .. self . 0 }) } # [doc = " Sets the value for explicit 1."] pub fn with_explicit_one_value (self , explicit_one : Option < T >) -> Self { Self (PluralElementsInner { explicit_one , .. self . 0 }) } }
};
}

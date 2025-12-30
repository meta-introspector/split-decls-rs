// Generated macro for fake_colors (module)
macro_rules! Depcrate_utilsfake_colors {
() => {
// Module: crate::utils
// Provides: {"fake_colors"}
// Dependencies: {}
# [cfg (not (feature = "colors"))] mod fake_colors { pub struct FakeStyledObject < D > (D) ; macro_rules ! style_attr { ($ ($ name : ident) *) => { $ (# [inline] pub fn $ name (self) -> FakeStyledObject < D > { self }) * } } impl < D > FakeStyledObject < D > { style_attr ! (red green yellow cyan bold dim underlined) ; } impl < D : std :: fmt :: Display > std :: fmt :: Display for FakeStyledObject < D > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (& self . 0 , f) } } pub fn style < D > (val : D) -> FakeStyledObject < D > { FakeStyledObject (val) } }
};
}

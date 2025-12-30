// Generated macro for impl_65 (impl)
macro_rules! Depcrate_color_contextimpl_65 {
() => {
// Module: crate::color_context
// Provides: {"impl_65"}
// Dependencies: {}
impl From < & [Change] > for ChangeSet { fn from (changes : & [Change]) -> ChangeSet { let mut change_set = ChangeSet :: default () ; for change in changes { match change { Change :: Foreground (color) => change_set . foreground = Some (color . clone ()) , Change :: Background (color) => change_set . background = Some (color . clone ()) , Change :: Bold => change_set . bold = true , Change :: Dim => change_set . dim = true , Change :: Underline => change_set . underline = true , Change :: Italics => change_set . italics = true , Change :: Blink => change_set . blink = true , Change :: Strike => change_set . strike = true , Change :: Reverse => change_set . reverse = true , Change :: Conceal => change_set . conceal = true , } } change_set } }
};
}

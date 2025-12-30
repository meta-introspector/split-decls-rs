// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl StatefulWidget for SelectedTab { type State = u16 ; fn render (self , area : Rect , buf : & mut Buffer , spacing : & mut Self :: State) { let spacing = * spacing ; match self { Self :: Legacy => Self :: render_examples (area , buf , Flex :: Legacy , spacing) , Self :: Start => Self :: render_examples (area , buf , Flex :: Start , spacing) , Self :: Center => Self :: render_examples (area , buf , Flex :: Center , spacing) , Self :: End => Self :: render_examples (area , buf , Flex :: End , spacing) , Self :: SpaceEvenly => Self :: render_examples (area , buf , Flex :: SpaceEvenly , spacing) , Self :: SpaceBetween => Self :: render_examples (area , buf , Flex :: SpaceBetween , spacing) , Self :: SpaceAround => Self :: render_examples (area , buf , Flex :: SpaceAround , spacing) , } } }
};
}
